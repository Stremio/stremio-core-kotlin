use std::io::Cursor;
use std::os::raw::c_void;
use std::panic;
use std::sync::RwLock;

use futures::{future, StreamExt};
use jni::objects::{JClass, JObject};
use jni::sys::{jbyteArray, jint, jobject, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM};
use lazy_static::lazy_static;
use prost::Message;
use stremio_core::constants::{
    LIBRARY_RECENT_STORAGE_KEY, LIBRARY_STORAGE_KEY, PROFILE_STORAGE_KEY,
};
use stremio_core::models::common::Loadable;
use stremio_core::runtime::{Env, EnvError, Runtime};
use stremio_core::types::library::LibraryBucket;
use stremio_core::types::profile::Profile;
use stremio_core::types::resource::Stream;

use crate::bridge::{FromProtobuf, ToProtobuf, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::ExceptionDescribeExt;
use crate::model::AndroidModel;
use crate::protobuf::stremio::core::runtime;
use crate::protobuf::stremio::core::runtime::Field;

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
                        let event_buf = event.to_protobuf(&()).encode_to_vec();
                        let event = env
                            .byte_array_from_slice(&event_buf)
                            .exception_describe(&env)
                            .expect("RuntimeEvent convert failed");
                        let _ = env
                            .call_static_method(
                                classes.get(&KotlinClassName::Core).unwrap(),
                                "onRuntimeEvent",
                                "([B)V",
                                &[event.into()],
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
    action_protobuf: jbyteArray,
) {
    let runtime_action = env
        .convert_byte_array(action_protobuf)
        .ok()
        .map(|data| Cursor::new(data))
        .and_then(|buf| runtime::RuntimeAction::decode(buf).ok())
        .map(|action| action.from_protobuf())
        .expect("Action convert failed");
    let runtime = RUNTIME.read().expect("RUNTIME read failed");
    let runtime = runtime
        .as_ref()
        .expect("RUNTIME not initialized")
        .as_ref()
        .expect("RUNTIME not initialized");
    runtime.dispatch(runtime_action);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_getStateBinary(
    env: JNIEnv,
    _class: JClass,
    field: JObject,
) -> jobject {
    let field = env
        .call_method(field, "getValue", "()I", &[])
        .and_then(|result| result.i())
        .ok()
        .and_then(|result| Field::from_i32(result).from_protobuf())
        .expect("AndroidModelField convert failed");
    let runtime = RUNTIME.read().expect("RUNTIME read failed");
    let runtime = runtime
        .as_ref()
        .expect("RUNTIME not initialized")
        .as_ref()
        .expect("RUNTIME not initialized");
    let model = runtime.model().expect("model read failed");
    let message_buf = model.get_state_binary(&field);
    env.byte_array_from_slice(&message_buf)
        .exception_describe(&env)
        .expect("state convert failed")
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_decodeStreamDataBinary(
    env: JNIEnv,
    _class: JClass,
    field: JObject,
) -> jobject {
    let stream_data = env
        .get_string(field.into())
        .exception_describe(&env)
        .expect("stream data convert failed")
        .to_string_lossy()
        .into_owned();
    let stream_buf = Stream::decode(stream_data)
        .map(|stream| stream.to_protobuf(&(None, None, None)))
        .map(|protobuf| protobuf.encode_to_vec())
        .expect("stream decoding failed");
    env.byte_array_from_slice(&stream_buf)
        .exception_describe(&env)
        .expect("stream decoding failed")
}
