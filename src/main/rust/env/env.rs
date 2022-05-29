use crate::env::{fetch, KotlinClassName, Storage};
use chrono::{DateTime, Utc};
use futures::{future, Future, TryFutureExt};
use http::Request;
use jni::objects::{GlobalRef, JObject};
use jni::JNIEnv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::{LockResult, RwLock, RwLockReadGuard};
use stremio_core::models::ctx::Ctx;
use stremio_core::models::streaming_server::StreamingServer;
use stremio_core::runtime::{Env, EnvError, EnvFuture, EnvFutureExt, TryEnvFuture};
use strum::IntoEnumIterator;

const INSTALLATION_ID_STORAGE_KEY: &str = "installation_id";
#[cfg(debug_assertions)]
const LOG_DEBUG_PRIORITY: i32 = 3;
#[cfg(debug_assertions)]
const LOG_TAG: &str = "AndroidEnv";

lazy_static! {
    static ref CONCURRENT_RUNTIME: RwLock<tokio::runtime::Runtime> = RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("CONCURRENT_RUNTIME_THREAD")
            .worker_threads(5)
            .enable_all()
            .build()
            .expect("CONCURRENT_RUNTIME create failed")
    );
    static ref SEQUENTIAL_RUNTIME: RwLock<tokio::runtime::Runtime> = RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("SEQUENTIAL_RUNTIME_THREAD")
            .enable_all()
            .build()
            .expect("SEQUENTIAL_RUNTIME create failed")
    );
    static ref KOTLIN_CLASSES: RwLock<HashMap<KotlinClassName, GlobalRef>> = Default::default();
    static ref STORAGE: RwLock<Option<Storage>> = Default::default();
    static ref INSTALLATION_ID: RwLock<Option<String>> = Default::default();
}

extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

pub enum AndroidEnv {}

impl AndroidEnv {
    pub fn init(env: &JNIEnv, storage: JObject) -> TryEnvFuture<()> {
        *KOTLIN_CLASSES.write().expect("KOTLIN_CLASSES write failed") =
            load_kotlin_classes(&env).expect("kotlin classes load failed");
        *STORAGE.write().expect("STORAGE write failed") =
            Some(Storage::new(&env, storage).expect("Create Storage failed"));
        AndroidEnv::migrate_storage_schema()
            .and_then(|_| async {
                let installation_id = get_installation_id().await?;
                *INSTALLATION_ID
                    .write()
                    .expect("INSTALLATION_ID write failed") = Some(installation_id);
                Ok(())
            })
            .boxed_env()
    }
    pub fn kotlin_classes<'a>(
    ) -> LockResult<RwLockReadGuard<'a, HashMap<KotlinClassName, GlobalRef>>> {
        KOTLIN_CLASSES.read()
    }
    pub fn exec_sync<F: Future>(future: F) -> F::Output {
        SEQUENTIAL_RUNTIME
            .read()
            .expect("SEQUENTIAL_RUNTIME read failed")
            .block_on(future)
    }
    pub fn random_buffer(len: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; len];
        getrandom::getrandom(buffer.as_mut_slice()).expect("getrandom failed");
        buffer
    }
}

impl Env for AndroidEnv {
    fn fetch<IN: Serialize, OUT: for<'de> Deserialize<'de> + Send + 'static>(
        request: Request<IN>,
    ) -> TryEnvFuture<OUT> {
        fetch(request)
    }
    fn get_storage<T: for<'de> Deserialize<'de> + Send + 'static>(
        key: &str,
    ) -> TryEnvFuture<Option<T>> {
        let storage = STORAGE.read().expect("STORAGE read failed");
        let storage = storage.as_ref().expect("STORAGE not initialized");
        storage.get::<T>(key)
    }
    fn set_storage<T: Serialize>(key: &str, value: Option<&T>) -> TryEnvFuture<()> {
        let storage = STORAGE.read().expect("STORAGE read failed");
        let storage = storage.as_ref().expect("STORAGE not initialized");
        storage.set::<T>(key, value)
    }
    fn exec_concurrent<F: Future<Output = ()> + Send + 'static>(future: F) {
        CONCURRENT_RUNTIME
            .read()
            .expect("CONCURRENT_RUNTIME read failed")
            .spawn(future);
    }
    fn exec_sequential<F: Future<Output = ()> + Send + 'static>(future: F) {
        SEQUENTIAL_RUNTIME
            .read()
            .expect("SEQUENTIAL_RUNTIME read failed")
            .spawn(future);
    }
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
    fn flush_analytics() -> EnvFuture<()> {
        future::ready(()).boxed_env()
    }
    fn analytics_context(_ctx: &Ctx, _streaming_server: &StreamingServer) -> serde_json::Value {
        serde_json::Value::Null
    }
    #[cfg(debug_assertions)]
    fn log(message: String) {
        unsafe {
            let tag = CString::new(LOG_TAG).unwrap();
            let message = CString::new(message).unwrap();
            __android_log_write(LOG_DEBUG_PRIORITY as c_int, tag.as_ptr(), message.as_ptr());
        }
    }
}

fn load_kotlin_classes(env: &JNIEnv) -> jni::errors::Result<HashMap<KotlinClassName, GlobalRef>> {
    KotlinClassName::iter()
        .map(|class_name| {
            env.find_class(class_name.value())
                .and_then(|class| env.new_global_ref(class))
                .map(|global_ref| (class_name, global_ref))
        })
        .collect::<Result<HashMap<_, _>, _>>()
}

async fn get_installation_id() -> Result<String, EnvError> {
    let installation_id = AndroidEnv::get_storage::<String>(INSTALLATION_ID_STORAGE_KEY).await?;
    let installation_id =
        installation_id.unwrap_or_else(|| hex::encode(AndroidEnv::random_buffer(10)));
    let _ = AndroidEnv::set_storage(INSTALLATION_ID_STORAGE_KEY, Some(&installation_id)).await?;
    Ok(installation_id)
}
