use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::{ExceptionDescribeExt, JObjectExt};
use crate::model::{AndroidModel, AndroidModelField};
use boolinator::Boolinator;
use futures::{future, StreamExt};
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobject, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM};
use lazy_static::lazy_static;
use std::os::raw::c_void;
use std::panic;
use std::sync::RwLock;
use stremio_core::constants::{
    LIBRARY_RECENT_STORAGE_KEY, LIBRARY_STORAGE_KEY, PROFILE_STORAGE_KEY,
};
use stremio_core::models::common::Loadable;
use stremio_core::runtime::msg::Action;
use stremio_core::runtime::{Env, EnvError, Runtime, RuntimeAction};
use stremio_core::types::library::LibraryBucket;
use stremio_core::types::profile::Profile;

lazy_static! {
    static ref RUNTIME: RwLock<Option<Loadable<Runtime<AndroidEnv, AndroidModel>, EnvError>>> =
        Default::default();
}

#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(_: JavaVM, _: *mut c_void) -> jint {
    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(|info| {
        AndroidEnv::log(info.to_string());
    }));
    // TODO: consider using ensure_local_capacity
    JNI_VERSION_1_6
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_initialize(
    env: JNIEnv,
    _class: JClass,
    storage: JObject,
) -> jobject {
    if RUNTIME.read().expect("RUNTIME read failed").is_some() {
        return JObject::null().into_inner();
    };

    let init_result = AndroidEnv::exec_sync(AndroidEnv::init(&env, storage));
    match init_result {
        Ok(_) => {
            let storage_result = AndroidEnv::exec_sync(future::try_join3(
                AndroidEnv::get_storage::<Profile>(PROFILE_STORAGE_KEY),
                AndroidEnv::get_storage::<LibraryBucket>(LIBRARY_RECENT_STORAGE_KEY),
                AndroidEnv::get_storage::<LibraryBucket>(LIBRARY_STORAGE_KEY),
            ));
            match storage_result {
                Ok((profile, recent_bucket, other_bucket)) => {
                    let profile = profile.unwrap_or_default();
                    let mut library = LibraryBucket::new(profile.uid(), vec![]);
                    if let Some(recent_bucket) = recent_bucket {
                        library.merge_bucket(recent_bucket);
                    };
                    if let Some(other_bucket) = other_bucket {
                        library.merge_bucket(other_bucket);
                    };
                    let (model, effects) = AndroidModel::new(profile, library);
                    let (runtime, rx) = Runtime::<AndroidEnv, _>::new(model, effects, 1000);
                    let java_vm = env.get_java_vm().expect("JavaVM get failed");
                    AndroidEnv::exec_concurrent(rx.for_each(move |event| {
                        let classes = AndroidEnv::kotlin_classes().unwrap();
                        let env = java_vm
                            .attach_current_thread_as_daemon()
                            .expect("JavaVM attach to current thread as deamon failed");
                        let event = event
                            .try_into_kotlin(&(), &env)
                            .exception_describe(&env)
                            .expect("RuntimeEvent convert failed")
                            .auto_local(&env);
                        let _ = env
                            .call_static_method(
                                classes.get(&KotlinClassName::Core).unwrap(),
                                "onRuntimeEvent",
                                format!("(L{};)V", KotlinClassName::RuntimeEvent.value()),
                                &[event.as_obj().into()],
                            )
                            .expect("onRuntimeEvent call failed");
                        future::ready(())
                    }));
                    *RUNTIME.write().expect("RUNTIME write failed") =
                        Some(Loadable::Ready(runtime));
                    JObject::null().into_inner()
                }
                Err(error) => {
                    *RUNTIME.write().expect("RUNTIME write failed") =
                        Some(Loadable::Err(error.to_owned()));
                    error
                        .try_into_kotlin(&(), &env)
                        .exception_describe(&env)
                        .expect("AndroidEnvError convert failed")
                        .into_inner()
                }
            }
        }
        Err(error) => {
            *RUNTIME.write().expect("RUNTIME write failed") = Some(Loadable::Err(error.to_owned()));
            error
                .try_into_kotlin(&(), &env)
                .exception_describe(&env)
                .expect("AndroidEnvError convert failed")
                .into_inner()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_dispatch(
    env: JNIEnv,
    _class: JClass,
    action: JObject,
    field: JObject,
) {
    let action = Action::try_from_kotlin(action, &env)
        .exception_describe(&env)
        .expect("Action convert failed");
    let field = (!field.is_null()).as_option().map(|_| {
        AndroidModelField::try_from_kotlin(field, &env)
            .exception_describe(&env)
            .expect("AndroidModelField convert failed")
    });
    let runtime = RUNTIME.read().expect("RUNTIME read failed");
    let runtime = runtime
        .as_ref()
        .expect("RUNTIME not initialized")
        .as_ref()
        .expect("RUNTIME not initialized");
    runtime.dispatch(RuntimeAction { action, field });
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_getState(
    env: JNIEnv,
    _class: JClass,
    field: JObject,
) -> jobject {
    let field = AndroidModelField::try_from_kotlin(field, &env)
        .exception_describe(&env)
        .expect("AndroidModelField convert failed");
    let runtime = RUNTIME.read().expect("RUNTIME read failed");
    let runtime = runtime
        .as_ref()
        .expect("RUNTIME not initialized")
        .as_ref()
        .expect("RUNTIME not initialized");
    let model = runtime.model().expect("model read failed");
    model
        .get_state(&field, &env)
        .exception_describe(&env)
        .expect("state convert failed")
        .into_inner()
}
