use std::fmt;

use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::models::common::Loadable;
use stremio_core::models::link::LinkError;
use stremio_core::models::streaming_server::Settings;
use stremio_core::runtime::EnvError;
use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use url::Url;

use crate::bridge::{ToProtobuf, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;

impl<'a, T, R: TryIntoKotlin<'a, T>, E: fmt::Display> TryIntoKotlin<'a, T> for Loadable<R, E> {
    #[inline]
    fn try_into_kotlin(&self, args: &T, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            Loadable::Ready(ready) => {
                let ready = ready.try_into_kotlin(args, env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Loadable_Ready).unwrap(),
                    "(Ljava/lang/Object;)V",
                    &[ready.as_obj().into()],
                )
            }
            Loadable::Err(error) => {
                let message = error.to_string().try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Loadable_Error).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[message.as_obj().into()],
                )
            }
            Loadable::Loading => env.new_object(
                classes.get(&KotlinClassName::Loadable_Loading).unwrap(),
                "()V",
                &[],
            ),
        }
    }
}

impl ToProtobuf<models::LoadableSettings, ()> for Loadable<Settings, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableSettings {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_settings::Content::Ready(ready.to_protobuf(&()))
            }
            Loadable::Err(error) => models::loadable_settings::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_settings::Content::Loading(models::Loading {}),
        };
        models::LoadableSettings {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableBaseUrl, ()> for Loadable<Url, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableBaseUrl {
        let content = match &self {
            Loadable::Ready(ready) => models::loadable_base_url::Content::Ready(ready.to_string()),
            Loadable::Err(error) => models::loadable_base_url::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_base_url::Content::Loading(models::Loading {}),
        };
        models::LoadableBaseUrl {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableCode, ()> for Loadable<LinkCodeResponse, LinkError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableCode {
        let content = match &self {
            Loadable::Ready(ready) => models::loadable_code::Content::Ready(ready.to_protobuf(&())),
            Loadable::Err(error) => models::loadable_code::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_code::Content::Loading(models::Loading {}),
        };
        models::LoadableCode {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableAuthKey, ()> for Loadable<LinkAuthKey, LinkError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableAuthKey {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_auth_key::Content::Ready(ready.to_protobuf(&()))
            }
            Loadable::Err(error) => models::loadable_auth_key::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_auth_key::Content::Loading(models::Loading {}),
        };
        models::LoadableAuthKey {
            content: Some(content),
        }
    }
}
