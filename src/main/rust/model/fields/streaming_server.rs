use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::streaming_server::{
    Selected as StreamingServerSelected, Settings as StreamingServerSettings, StreamingServer,
};

use crate::bridge::{FromProtobuf, ToProtobuf, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;

impl TryFromKotlin for StreamingServerSettings {
    fn try_from_kotlin<'a>(settings: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let app_path = env
            .call_method(
                settings,
                "getAppPath",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let app_path = String::try_from_kotlin(app_path.as_obj(), env)?;
        let cache_root = env
            .call_method(
                settings,
                "getCacheRoot",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let cache_root = String::try_from_kotlin(cache_root.as_obj(), env)?;
        let server_version = env
            .call_method(
                settings,
                "getServerVersion",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let server_version = String::try_from_kotlin(server_version.as_obj(), env)?;
        let cache_size = env
            .call_method(
                settings,
                "getCacheSize",
                format!("()L{};", "java/lang/Double"),
                &[],
            )?
            .l()?
            .auto_local(env);
        let cache_size = cache_size.as_obj();
        let cache_size = if cache_size.is_null() {
            None
        } else {
            let cache_size = env
                .call_method(cache_size, "doubleValue", "()D", &[])?
                .d()?;
            Some(cache_size)
        };
        let bt_max_connections = env
            .call_method(settings, "getBtMaxConnections", "()J", &[])?
            .j()? as u64;
        let bt_handshake_timeout = env
            .call_method(settings, "getBtHandshakeTimeout", "()J", &[])?
            .j()? as u64;
        let bt_request_timeout = env
            .call_method(settings, "getBtRequestTimeout", "()J", &[])?
            .j()? as u64;
        let bt_download_speed_soft_limit = env
            .call_method(settings, "getBtDownloadSpeedSoftLimit", "()D", &[])?
            .d()?;
        let bt_download_speed_hard_limit = env
            .call_method(settings, "getBtDownloadSpeedHardLimit", "()D", &[])?
            .d()?;
        let bt_min_peers_for_stable = env
            .call_method(settings, "getBtMinPeersForStable", "()J", &[])?
            .j()? as u64;
        Ok(StreamingServerSettings {
            app_path,
            cache_root,
            server_version,
            cache_size,
            bt_max_connections,
            bt_handshake_timeout,
            bt_request_timeout,
            bt_download_speed_soft_limit,
            bt_download_speed_hard_limit,
            bt_min_peers_for_stable,
        })
    }
}

impl FromProtobuf<StreamingServerSettings> for models::streaming_server::Settings {
    fn from_protobuf(&self) -> StreamingServerSettings {
        StreamingServerSettings {
            app_path: self.app_path.to_owned(),
            cache_root: self.cache_root.to_owned(),
            server_version: self.server_version.to_owned(),
            cache_size: self.cache_size.to_owned(),
            bt_max_connections: self.bt_max_connections,
            bt_handshake_timeout: self.bt_handshake_timeout,
            bt_request_timeout: self.bt_request_timeout,
            bt_download_speed_soft_limit: self.bt_download_speed_soft_limit,
            bt_download_speed_hard_limit: self.bt_download_speed_hard_limit,
            bt_min_peers_for_stable: self.bt_min_peers_for_stable,
        }
    }
}

impl ToProtobuf<models::streaming_server::Selected, ()> for StreamingServerSelected {
    fn to_protobuf(&self, _args: &()) -> models::streaming_server::Selected {
        models::streaming_server::Selected {
            transport_url: self.transport_url.to_string(),
        }
    }
}

impl ToProtobuf<models::streaming_server::Settings, ()> for StreamingServerSettings {
    fn to_protobuf(&self, _args: &()) -> models::streaming_server::Settings {
        models::streaming_server::Settings {
            app_path: self.app_path.to_string(),
            cache_root: self.cache_root.to_string(),
            server_version: self.server_version.to_string(),
            cache_size: self.cache_size.clone(),
            bt_max_connections: self.bt_max_connections,
            bt_handshake_timeout: self.bt_handshake_timeout,
            bt_request_timeout: self.bt_request_timeout,
            bt_download_speed_soft_limit: self.bt_download_speed_soft_limit,
            bt_download_speed_hard_limit: self.bt_download_speed_hard_limit,
            bt_min_peers_for_stable: self.bt_min_peers_for_stable,
        }
    }
}

impl ToProtobuf<models::StreamingServer, ()> for StreamingServer {
    fn to_protobuf(&self, _args: &()) -> models::StreamingServer {
        models::StreamingServer {
            selected: self.selected.to_protobuf(&()),
            settings: self.settings.to_protobuf(&()),
            base_url: self.base_url.to_protobuf(&()),
        }
    }
}
