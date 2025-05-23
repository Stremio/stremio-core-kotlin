// TODO: Add safety docs and remove suppression of linter!
#![allow(clippy::missing_safety_doc)]

#[cfg(debug_assertions)]
use std::panic;
use std::{io::Cursor, os::raw::c_void, sync::RwLock};

use enclose::enclose;
use futures::{future, StreamExt};
use jni::{
    objects::{JClass, JObject},
    sys::{jbyteArray, jint, jobject, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};

use once_cell::sync::Lazy;
use prost::Message;

use stremio_core::{
    constants::{
        DISMISSED_EVENTS_STORAGE_KEY, LIBRARY_RECENT_STORAGE_KEY, LIBRARY_STORAGE_KEY,
        NOTIFICATIONS_STORAGE_KEY, PROFILE_STORAGE_KEY, SEARCH_HISTORY_STORAGE_KEY,
        STREAMING_SERVER_URLS_STORAGE_KEY, STREAMS_STORAGE_KEY,
    },
    models::common::Loadable,
    runtime::{Env, EnvError, Runtime, RuntimeEvent},
    types::{
        events::DismissedEventsBucket, library::LibraryBucket, notifications::NotificationsBucket,
        profile::Profile, resource::Stream, search_history::SearchHistoryBucket,
        server_urls::ServerUrlsBucket, streams::StreamsBucket,
    },
};
use stremio_core_protobuf::{FromProtobuf, ToProtobuf};

use crate::{
    bridge::ToJNIByteArray,
    env::{AndroidEnv, AndroidEvent, KotlinClassName},
    jni_ext::ExceptionDescribeExt,
    model::AndroidModel,
    protobuf::stremio::core::runtime::{self, Field},
};

#[allow(clippy::type_complexity)]
static RUNTIME: Lazy<RwLock<Option<Loadable<Runtime<AndroidEnv, AndroidModel>, EnvError>>>> =
    Lazy::new(Default::default);

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
pub unsafe extern "C" fn Java_com_stremio_core_Core_initializeNative(
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
            let storage_result = AndroidEnv::exec_sync(future::try_join4(
                future::try_join5(
                    AndroidEnv::get_storage::<Profile>(PROFILE_STORAGE_KEY),
                    AndroidEnv::get_storage::<LibraryBucket>(LIBRARY_RECENT_STORAGE_KEY),
                    AndroidEnv::get_storage::<LibraryBucket>(LIBRARY_STORAGE_KEY),
                    AndroidEnv::get_storage::<StreamsBucket>(STREAMS_STORAGE_KEY),
                    AndroidEnv::get_storage::<ServerUrlsBucket>(STREAMING_SERVER_URLS_STORAGE_KEY),
                ),
                AndroidEnv::get_storage::<NotificationsBucket>(NOTIFICATIONS_STORAGE_KEY),
                AndroidEnv::get_storage::<SearchHistoryBucket>(SEARCH_HISTORY_STORAGE_KEY),
                AndroidEnv::get_storage::<DismissedEventsBucket>(DISMISSED_EVENTS_STORAGE_KEY),
            ));
            match storage_result {
                Ok((
                    (profile, recent_bucket, other_bucket, streams, server_urls_bucket),
                    notifications,
                    search_history,
                    dismissed_events,
                )) => {
                    let profile = profile.unwrap_or_default();
                    let mut library = LibraryBucket::new(profile.uid(), vec![]);
                    if let Some(recent_bucket) = recent_bucket {
                        library.merge_bucket(recent_bucket);
                    };
                    if let Some(other_bucket) = other_bucket {
                        library.merge_bucket(other_bucket);
                    };
                    let streams = streams.unwrap_or(StreamsBucket::new(profile.uid()));
                    let server_urls_bucket =
                        server_urls_bucket
                            .unwrap_or(ServerUrlsBucket::new::<AndroidEnv>(profile.uid()));
                    let notifications = notifications.unwrap_or(NotificationsBucket::new::<
                        AndroidEnv,
                    >(
                        profile.uid(), vec![]
                    ));
                    let search_history =
                        search_history.unwrap_or(SearchHistoryBucket::new(profile.uid()));
                    let dismissed_events =
                        dismissed_events.unwrap_or(DismissedEventsBucket::new(profile.uid()));
                    let (model, effects) = AndroidModel::new(
                        profile,
                        library,
                        streams,
                        server_urls_bucket,
                        notifications,
                        search_history,
                        dismissed_events,
                    );
                    let (runtime, rx) = Runtime::<AndroidEnv, _>::new(
                        model,
                        effects.into_iter().collect::<Vec<_>>(),
                        1000,
                    );
                    let java_vm = env.get_java_vm().expect("JavaVM get failed");
                    AndroidEnv::exec_concurrent(rx.for_each(move |event| {
                        if let RuntimeEvent::CoreEvent(event) = &event {
                            AndroidEnv::exec_concurrent(enclose!((event) async move {
                                let runtime = RUNTIME.read().expect("runtime read failed");
                                let runtime = runtime
                                    .as_ref()
                                    .expect("runtime is not ready")
                                    .as_ref()
                                    .expect("runtime is not ready");
                                let model = runtime.model().expect("model read failed");
                                AndroidEnv::emit_to_analytics(
                                    &AndroidEvent::CoreEvent(event.to_owned()),
                                    &model,
                                    "TODO"
                                );
                            }));
                        };
                        let classes = AndroidEnv::kotlin_classes().unwrap();
                        let env = java_vm
                            .attach_current_thread_as_daemon()
                            .expect("JavaVM attach to current thread as deamon failed");
                        let event = event
                            .to_protobuf::<AndroidEnv>(&())
                            .encode_to_vec()
                            .to_jni_byte_array(&env);
                        let event = env.auto_local(event);
                        let _ = env
                            .call_static_method(
                                classes.get(&KotlinClassName::Core).unwrap(),
                                "onRuntimeEvent",
                                "([B)V",
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
                        .to_protobuf::<AndroidEnv>(&())
                        .encode_to_vec()
                        .to_jni_byte_array(&env)
                }
            }
        }
        Err(error) => {
            *RUNTIME.write().expect("RUNTIME write failed") = Some(Loadable::Err(error.to_owned()));
            error
                .to_protobuf::<AndroidEnv>(&())
                .encode_to_vec()
                .to_jni_byte_array(&env)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_dispatchNative(
    env: JNIEnv,
    _class: JClass,
    action_protobuf: jbyteArray,
) {
    let runtime_action = env
        .convert_byte_array(action_protobuf)
        .ok()
        .map(Cursor::new)
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
pub unsafe extern "C" fn Java_com_stremio_core_Core_getStateNative(
    env: JNIEnv,
    _class: JClass,
    field: JObject,
) -> jobject {
    let field = env
        .call_method(field, "getValue", "()I", &[])
        .and_then(|result| result.i())
        .ok()
        .and_then(|result| Field::try_from(result).ok().from_protobuf())
        .expect("AndroidModelField convert failed");
    let runtime = RUNTIME.read().expect("RUNTIME read failed");
    let runtime = runtime
        .as_ref()
        .expect("RUNTIME not initialized")
        .as_ref()
        .expect("RUNTIME not initialized");
    let model = runtime.model().expect("model read failed");
    model.get_state_binary(&field).to_jni_byte_array(&env)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_decodeStreamDataNative(
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
    let stream = match Stream::decode(stream_data) {
        Ok(stream) => stream,
        Err(_) => return JObject::null().into_inner(),
    };
    stream
        .to_protobuf::<AndroidEnv>(&(None, None, None, None))
        .encode_to_vec()
        .to_jni_byte_array(&env)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_stremio_core_Core_sendNextAnalyticsBatch(
    _env: JNIEnv,
    _class: JClass,
) {
    AndroidEnv::exec_concurrent(AndroidEnv::send_next_analytics_batch());
}
