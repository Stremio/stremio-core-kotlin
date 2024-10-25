use std::{
    collections::HashMap,
    os::raw::{c_char, c_int},
    sync::{LockResult, RwLock, RwLockReadGuard},
};

use chrono::{DateTime, Utc};
use futures::Future;
use http::Request;
use jni::{
    objects::{GlobalRef, JObject},
    JNIEnv,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::IntoEnumIterator;

use stremio_core::{
    analytics::Analytics,
    models::{ctx::Ctx, streaming_server::StreamingServer},
    runtime::{msg::Event, Env, EnvError, EnvFuture, EnvFutureExt, TryEnvFuture},
};

use crate::{
    env::{fetch, AndroidEvent, KotlinClassName, Storage},
    model::AndroidModel,
};

const INSTALLATION_ID_STORAGE_KEY: &str = "installation_id";
#[cfg(debug_assertions)]
const LOG_DEBUG_PRIORITY: i32 = 3;
#[cfg(debug_assertions)]
const LOG_TAG: &str = "AndroidEnv";

static CONCURRENT_RUNTIME: Lazy<RwLock<tokio::runtime::Runtime>> = Lazy::new(|| {
    RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("CONCURRENT_RUNTIME_THREAD")
            .worker_threads(30)
            .enable_all()
            .build()
            .expect("CONCURRENT_RUNTIME create failed"),
    )
});
static SEQUENTIAL_RUNTIME: Lazy<RwLock<tokio::runtime::Runtime>> = Lazy::new(|| {
    RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("SEQUENTIAL_RUNTIME_THREAD")
            .enable_all()
            .build()
            .expect("SEQUENTIAL_RUNTIME create failed"),
    )
});
static KOTLIN_CLASSES: Lazy<RwLock<HashMap<KotlinClassName, GlobalRef>>> =
    Lazy::new(Default::default);
static STORAGE: Lazy<RwLock<Option<Storage>>> = Lazy::new(Default::default);
static ANALYTICS: Lazy<Analytics<AndroidEnv>> = Lazy::new(Default::default);
static INSTALLATION_ID: Lazy<RwLock<Option<String>>> = Lazy::new(Default::default);
static VISIT_ID: Lazy<String> = Lazy::new(|| hex::encode(AndroidEnv::random_buffer(10)));

extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalyticsContext {
    app_type: String,
    app_version: String,
    server_version: Option<String>,
    shell_version: Option<String>,
    system_language: Option<String>,
    app_language: String,
    #[serde(rename = "installationID")]
    installation_id: String,
    #[serde(rename = "visitID")]
    visit_id: String,
    #[serde(rename = "url")]
    path: String,
}

pub enum AndroidEnv {}

impl AndroidEnv {
    pub fn init(env: &JNIEnv, storage: JObject) -> TryEnvFuture<()> {
        *KOTLIN_CLASSES.write().expect("KOTLIN_CLASSES write failed") =
            load_kotlin_classes(env).expect("kotlin classes load failed");
        *STORAGE.write().expect("STORAGE write failed") =
            Some(Storage::new(env, storage).expect("Create Storage failed"));

        async {
            Self::migrate_storage_schema().await?;

            let installation_id = get_installation_id().await?;
            *INSTALLATION_ID
                .write()
                .expect("INSTALLATION_ID write failed") = Some(installation_id);
            Ok(())
        }
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
    pub fn emit_to_analytics(event: &AndroidEvent, model: &AndroidModel, path: &str) {
        let (name, data) = match event {
            AndroidEvent::CoreEvent(Event::PlayerPlaying { load_time, context }) => (
                "playerPlaying".to_owned(),
                json!({
                    "loadTime": load_time,
                    "player": context
                }),
            ),
            AndroidEvent::CoreEvent(Event::PlayerStopped { context }) => {
                ("playerStopped".to_owned(), json!({ "player": context }))
            }
            AndroidEvent::CoreEvent(Event::PlayerEnded {
                context,
                is_binge_enabled,
                is_playing_next_video,
            }) => (
                "playerEnded".to_owned(),
                json!({
                   "player": context,
                   "isBingeEnabled": is_binge_enabled,
                   "isPlayingNextVideo": is_playing_next_video
                }),
            ),
            AndroidEvent::CoreEvent(Event::TraktPlaying { context }) => {
                ("traktPlaying".to_owned(), json!({ "player": context }))
            }
            AndroidEvent::CoreEvent(Event::TraktPaused { context }) => {
                ("traktPaused".to_owned(), json!({ "player": context }))
            }
            _ => return,
        };
        ANALYTICS.emit(name, data, &model.ctx, &model.streaming_server, path);
    }
    pub fn send_next_analytics_batch() -> impl Future<Output = ()> {
        ANALYTICS.send_next_batch()
    }
    pub fn random_buffer(len: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; len];
        getrandom::getrandom(buffer.as_mut_slice()).expect("getrandom failed");
        buffer
    }
}

impl Env for AndroidEnv {
    fn fetch<IN: Serialize + Send + 'static, OUT: for<'de> Deserialize<'de> + Send + 'static>(
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
    fn flush_analytics() -> EnvFuture<'static, ()> {
        ANALYTICS.flush().boxed_env()
    }
    fn analytics_context(
        ctx: &Ctx,
        streaming_server: &StreamingServer,
        path: &str,
    ) -> serde_json::Value {
        serde_json::to_value(AnalyticsContext {
            app_type: "android-tv".to_owned(),
            app_version: "TODO".to_owned(),
            server_version: streaming_server
                .settings
                .as_ref()
                .ready()
                .map(|settings| settings.server_version.to_owned()),
            shell_version: None,
            system_language: Some("TODO".to_owned()),
            app_language: ctx.profile.settings.interface_language.to_owned(),
            installation_id: INSTALLATION_ID
                .read()
                .expect("installation id read failed")
                .as_ref()
                .expect("installation id not available")
                .to_owned(),
            visit_id: VISIT_ID.to_owned(),
            path: path.to_owned(),
        })
        .unwrap()
    }
    #[cfg(debug_assertions)]
    fn log(message: String) {
        use std::ffi::CString;
        let tag = CString::new(LOG_TAG).unwrap();
        let message = CString::new(message).unwrap();

        unsafe {
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
    AndroidEnv::set_storage(INSTALLATION_ID_STORAGE_KEY, Some(&installation_id)).await?;
    Ok(installation_id)
}
