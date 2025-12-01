// TODO: Add safety docs and remove suppression of linter!
#![allow(clippy::missing_safety_doc)]

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
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::{
    bridge::ToJNIByteArray,
    env::{android_log_write, AndroidEnv, AndroidEvent, KotlinClassName, PANIC_LOG_TAG},
    jni_ext::ExceptionDescribeExt,
    model::AndroidModel,
    protobuf::stremio::core::runtime::{self, Field},
    subscriber::KotlinLayerConfigBuilder,
};

pub mod subscriber;

#[allow(clippy::type_complexity)]
static RUNTIME: Lazy<RwLock<Option<Loadable<Runtime<AndroidEnv, AndroidModel>, EnvError>>>> =
    Lazy::new(Default::default);

/// Initialize panic hook to send data to Kotlin
#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(_: JavaVM, _: *mut c_void) -> jint {
    std::panic::set_hook(Box::new(|info| {
        let info_str = info.to_string();
        eprintln!("FATAL: {}", &info_str);
        // Attempt to set the flag from false to true.
        let _ret = android_log_write(
            crate::env::AndroidLogPriority::Fatal,
            PANIC_LOG_TAG,
            &info_str,
        )
        .inspect_err(|err| {
            eprintln!("Failed to log PANIC log with AndroidLogPriority::Fatal: {err}")
        });
    }));

    let env_filter = EnvFilter::builder().from_env_lossy();
    let max_level_hint = env_filter.max_level_hint();

    #[cfg(any(debug_assertions, feature = "log-trace"))]
    let max_level = tracing::Level::TRACE;
    #[cfg(all(not(debug_assertions), not(feature = "log-trace")))]
    let max_level = tracing::Level::INFO;

    let max_level = max_level_hint
        .and_then(|level_filter| level_filter.into_level())
        .unwrap_or(max_level);

    let config = KotlinLayerConfigBuilder::default()
        .set_max_level(max_level)
        .build();

    // setup Swift tracing Subscriber
    subscriber::set_as_global_default_with_config(config);

    info!(?max_level, "Logging level");

    // TODO: consider using ensure_local_capacity
    JNI_VERSION_1_6
}

#[no_mangle]
/// Initializes core and starts the Runtime. Run only once!
///
/// # Returns
///
/// - `JObject::null()` - if initializeNative has already been called once.
pub unsafe extern "C" fn Java_com_stremio_core_Core_initializeNative(
    env: JNIEnv,
    _class: JClass,
    storage: JObject,
) -> jobject {
    if RUNTIME
        .read()
        .inspect_err(|err| tracing::error!(runtime = ?err, context="initializeNative", "RUNTIME read failed due to poisoning"))
        .ok()
        .map(|runtime| runtime.as_ref().is_some())
        .unwrap_or(false)
    {
        tracing::warn!(context="initializeNative", "RUNTIME already initialized Some(Runtime)");

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
                                let handle_event = || -> Option<bool> {
                                    let lock = RUNTIME.read().inspect_err(|err| tracing::error!(context="Receive core event", runtime = ?err, "RUNTIME read failed")).ok()?;

                                    let runtime = lock
                                        .as_ref()
                                        .map_or_else(|| {
                                            tracing::error!(runtime = "None", context="Receive core event", "RUNTIME hasn't been not initialized");

                                            None
                                        }, |runtime_initialized| match runtime_initialized.as_ref() {
                                            Loadable::Ready(runtime) => Some(runtime),
                                            Loadable::Loading => {
                                                tracing::warn!(runtime = ?Loadable::<(), EnvError>::Loading, context="Receive core event", "Runtime is still initializing (Loadable::Loading)");

                                                None
                                            },
                                            Loadable::Err(err) => {
                                                tracing::error!(runtime = ?Loadable::<(), _>::Err(err), context="Receive core event", "Runtime initialization error");
                                                None
                                            }
                                        })?;

                                    let model = runtime.model().inspect_err(|err| {
                                        tracing::error!(model = ?err, context="Receive core event", "Runtime model read failed due to poisoning")
                                    }).ok()?;

                                    AndroidEnv::emit_to_analytics(
                                        &AndroidEvent::CoreEvent(event.to_owned()),
                                        &model,
                                        "TODO"
                                    );

                                    Some(true)
                                };

                                let _option = handle_event();
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
) -> jobject {
    let runtime_action = env
        .convert_byte_array(action_protobuf)
        .map_err(|err| {
            EnvError::Other(format!(
                "dispatchNative: Couldn't convert a java byte array to a rust vector of bytes: {err}"
            ))
        })
        .and_then(|bytes| {
            let buf = Cursor::new(bytes);

            match runtime::RuntimeAction::decode(buf) {
            Ok(action) => Ok(action.from_protobuf()),
            Err(err) => Err(EnvError::Other(format!(
                "dispatchNative: Couldn't decode Runtime Action: {err}"
            )))
        }
    });

    let runtime_action = match runtime_action {
        Ok(x) => x,
        // return early with error encoded to protobuf
        Err(err) => {
            return err
                .to_protobuf::<AndroidEnv>(&())
                .encode_to_vec()
                .to_jni_byte_array(&env)
        }
    };
    let runtime = RUNTIME.read().expect("RUNTIME read failed");

    match runtime.as_ref() {
        Some(Loadable::Loading) => {
            error!(
                function = "dispatchNative",
                "Runtime initialization hasn't loaded yet (Loadable::Loading)"
            );
            EnvError::Other(
                "dispatchNative: Runtime initialization hasn't loaded yet (Loadable::Loading)"
                    .to_string(),
            )
            .to_protobuf::<AndroidEnv>(&())
            .encode_to_vec()
            .to_jni_byte_array(&env)
        }
        Some(Loadable::Err(err)) => {
            error!(
                function = "dispatchNative",
                "Runtime initialization hasn't errored (Loadable::Error): {err}"
            );

            EnvError::Other(format!(
                "dispatchNative: Runtime initialization hasn't errored (Loadable::Error): {err}"
            ))
            .to_protobuf::<AndroidEnv>(&())
            .encode_to_vec()
            .to_jni_byte_array(&env)
        }
        Some(Loadable::Ready(runtime)) => {
            runtime.dispatch(runtime_action);

            JObject::null().into_inner()
        }
        None => {
            error!(
                function = "dispatchNative",
                "Runtime initialization is not set yet (None)"
            );
            EnvError::Other(
                "dispatchNative: Runtime initialization is not set yet (None)".to_string(),
            )
            .to_protobuf::<AndroidEnv>(&())
            .encode_to_vec()
            .to_jni_byte_array(&env)
        }
    }
}

#[no_mangle]
/// Will return null if core hasn't been initialized yet
pub unsafe extern "C" fn Java_com_stremio_core_Core_getStateNative(
    env: JNIEnv,
    _class: JClass,
    field: JObject,
) -> jobject {
    let field = env
        .call_method(field, "getValue", "()I", &[])
        .and_then(|result| result.i())
        .ok()
        .and_then(|result| Field::try_from(result).inspect_err(|err| {
            error!("Field (rust) failed to be parsed from the Field.getValue() (kotlin) passed value: {err}")
        }).ok().from_protobuf())
        .expect("AndroidModelField convert failed");
    let runtime = RUNTIME
        .read()
        .inspect_err(|_err| {
            error!("Runtime read failed due to RwLock poisoning");
        })
        .expect("RUNTIME read failed");

    match runtime.as_ref() {
        Some(Loadable::Loading) => {
            error!(
                function = "getStateNative",
                "Runtime initialization hasn't loaded yet (Loadable::Loading)"
            );

            JObject::null().into_inner()
        }
        Some(Loadable::Err(err)) => {
            error!(
                function = "getStateNative",
                "Runtime initialization hasn't errored (Loadable::Error): {err}"
            );
            JObject::null().into_inner()
        }
        Some(Loadable::Ready(runtime)) => {
            let model = runtime
                .model()
                .inspect_err(|_err| {
                    error!(
                        function = "getStateNative",
                        "Runtime RwLock model read has failed due to poisoning"
                    );
                })
                .expect("model read failed");

            model.get_state_binary(&field).to_jni_byte_array(&env)
        }
        None => {
            error!(
                function = "getStateNative",
                "Runtime initialization is not set yet (None)"
            );
            JObject::null().into_inner()
        }
    }
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
        .to_protobuf::<AndroidEnv>(&(None, None, None, None, None))
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
