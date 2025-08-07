use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use futures::future;
use jni::objects::{GlobalRef, JObject};
use jni::{JNIEnv, JavaVM};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::convert::{Into, TryInto};
use std::sync::Arc;
use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

pub struct Storage {
    storage: GlobalRef,
    java_vm: Arc<JavaVM>,
}

impl Storage {
    pub fn new(env: &JNIEnv, storage: JObject) -> jni::errors::Result<Self> {
        Ok(Self {
            storage: env.new_global_ref(storage)?,
            java_vm: Arc::new(env.get_java_vm()?),
        })
    }
    pub fn get<T: for<'de> Deserialize<'de> + Send + 'static>(
        &self,
        key: &str,
    ) -> TryEnvFuture<Option<T>> {
        let key = key.to_owned();
        let java_vm = self.java_vm.clone();
        let storage = self.storage.clone();
        future::lazy(move |_| {
            let env = java_vm
                .attach_current_thread_permanently()
                .map_err(|error| EnvError::Other(error.to_string()))?;
            let key = env
                .new_string(key)
                .map_err(|error| EnvError::Other(error.to_string()))?;
            let key = env.auto_local(key);
            let storage_result = env
                .call_method(
                    storage.as_obj(),
                    "get",
                    format!(
                        "(Ljava/lang/String;)L{};",
                        KotlinClassName::Storage_Result.value()
                    ),
                    &[key.as_obj().into()],
                )
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                .l()
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
            let storage_result = env.auto_local(storage_result);
            let storage_result_class_name = storage_result
                .as_obj()
                .get_class_name(&env)
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
            match storage_result_class_name.replace('.', "/").try_into() {
                Ok(KotlinClassName::Storage_Result_Ok) => {
                    let value = env
                        .call_method(
                            storage_result.as_obj(),
                            "getValue",
                            "()Ljava/lang/Object;",
                            &[],
                        )
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                        .l()
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
                    let value = env.auto_local(value);
                    if !value.as_obj().is_null() {
                        let value = env
                            .get_string(value.as_obj().into())
                            .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                            .to_string_lossy()
                            .into_owned();
                        let mut deserializer = Deserializer::from_str(&value);
                        cfg_if::cfg_if! {
                            if #[cfg(debug_assertions)] {
                                let value = serde_path_to_error::deserialize::<_, T>(&mut deserializer).map_err(|error| EnvError::Serde(error.to_string()))?;
                            } else {
                                let value = T::deserialize(&mut deserializer).map_err(|error| EnvError::Serde(error.to_string()))?;
                            }
                        };
                        Ok(Some(value))
                    } else {
                        Ok(None)
                    }
                }
                Ok(KotlinClassName::Storage_Result_Err) => {
                    let message = env
                        .call_method(
                            storage_result.as_obj(),
                            "getMessage",
                            "()Ljava/lang/String;",
                            &[],
                        )
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                        .l()
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
                    let message = env.auto_local(message);
                    let message = env
                        .get_string(message.as_obj().into())
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                        .to_string_lossy()
                        .into_owned();
                    Err(EnvError::StorageReadError(message))
                }
                _ => Err(EnvError::StorageReadError(format!(
                    "Invalid Storage$Result: {storage_result_class_name}"
                ))),
            }
        })
        .boxed_env()
    }
    pub fn set<T: Serialize>(&self, key: &str, value: Option<&T>) -> TryEnvFuture<()> {
        let key = key.to_owned();
        let value = value.map(|value| serde_json::to_string(&value));
        let value = match value.transpose() {
            Ok(value) => value,
            Err(error) => return future::err(EnvError::Serde(error.to_string())).boxed_env(),
        };
        let java_vm = self.java_vm.clone();
        let storage = self.storage.clone();
        future::lazy(move |_| {
            let env = java_vm
                .attach_current_thread_permanently()
                .map_err(|error| EnvError::Other(error.to_string()))?;
            let key = env
                .new_string(key)
                .map_err(|error| EnvError::Other(error.to_string()))?;
            let key = env.auto_local(key);
            let value = match value {
                Some(value) => env
                    .new_string(&value)
                    .map_err(|error| EnvError::Other(error.to_string()))?
                    .into(),
                _ => JObject::null(),
            };
            let value = env.auto_local(value);
            let storage_result = env
                .call_method(
                    storage.as_obj(),
                    "set",
                    format!(
                        "(Ljava/lang/String;Ljava/lang/String;)L{};",
                        KotlinClassName::Storage_Result.value()
                    ),
                    &[key.as_obj().into(), value.as_obj().into()],
                )
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                .l()
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
            let storage_result = env.auto_local(storage_result);
            let storage_result_class_name = storage_result
                .as_obj()
                .get_class_name(&env)
                .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
            match storage_result_class_name.replace('.', "/").try_into() {
                Ok(KotlinClassName::Storage_Result_Ok) => Ok(()),
                Ok(KotlinClassName::Storage_Result_Err) => {
                    let message = env
                        .call_method(
                            storage_result.as_obj(),
                            "getMessage",
                            "()Ljava/lang/String;",
                            &[],
                        )
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                        .l()
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?;
                    let message = env.auto_local(message);
                    let message = env
                        .get_string(message.as_obj().into())
                        .map_err(|error| EnvError::StorageReadError(error.to_string()))?
                        .to_string_lossy()
                        .into_owned();
                    Err(EnvError::StorageReadError(message))
                }
                _ => Err(EnvError::StorageReadError(format!(
                    "Invalid Storage$Result: {storage_result_class_name}"
                ))),
            }
        })
        .boxed_env()
    }
}
