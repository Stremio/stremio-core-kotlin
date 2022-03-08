use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::streaming_server::{
    Selected as StreamingServerSelected, Settings as StreamingServerSettings, StreamingServer,
};

impl<'a> TryIntoKotlin<'a, ()> for StreamingServerSelected {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let transport_url = self
            .transport_url
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::StreamingServer_Selected)
                .unwrap(),
            "(Ljava/lang/String;)V",
            &[transport_url.as_obj().into()],
        )
    }
}

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
            .call_method(settings, "getBtMaxConnections-s-VKNKU", "()J", &[])?
            .j()? as u64;
        let bt_handshake_timeout = env
            .call_method(settings, "getBtHandshakeTimeout-s-VKNKU", "()J", &[])?
            .j()? as u64;
        let bt_request_timeout = env
            .call_method(settings, "getBtRequestTimeout-s-VKNKU", "()J", &[])?
            .j()? as u64;
        let bt_download_speed_soft_limit = env
            .call_method(settings, "getBtDownloadSpeedSoftLimit", "()D", &[])?
            .d()?;
        let bt_download_speed_hard_limit = env
            .call_method(settings, "getBtDownloadSpeedHardLimit", "()D", &[])?
            .d()?;
        let bt_min_peers_for_stable = env
            .call_method(settings, "getBtMinPeersForStable-s-VKNKU", "()J", &[])?
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

impl<'a> TryIntoKotlin<'a, ()> for StreamingServerSettings {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let app_path = self.app_path.try_into_kotlin(&(), env)?.auto_local(env);
        let cache_root = self.cache_root.try_into_kotlin(&(), env)?.auto_local(env);
        let server_version = self
            .server_version
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let cache_size = match self.cache_size {
            Some(cache_size) => env.new_object("java/lang/Double", "(D)V", &[cache_size.into()])?,
            _ => JObject::null(),
        };
        let cache_size = env.auto_local(cache_size);
        let bt_max_connections = (self.bt_max_connections as i64).into();
        let bt_handshake_timeout = (self.bt_handshake_timeout as i64).into();
        let bt_request_timeout = (self.bt_request_timeout as i64).into();
        let bt_download_speed_soft_limit = self.bt_download_speed_soft_limit.into();
        let bt_download_speed_hard_limit = self.bt_download_speed_hard_limit.into();
        let bt_min_peers_for_stable = (self.bt_min_peers_for_stable as i64).into();
        env.new_object(
            classes
                .get(&KotlinClassName::StreamingServer_Settings)
                .unwrap(),
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Double;JJJDDJ)V",
            &[
                app_path.as_obj().into(),
                cache_root.as_obj().into(),
                server_version.as_obj().into(),
                cache_size.as_obj().into(),
                bt_max_connections,
                bt_handshake_timeout,
                bt_request_timeout,
                bt_download_speed_soft_limit,
                bt_download_speed_hard_limit,
                bt_min_peers_for_stable,
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for StreamingServer {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let selected = self.selected.try_into_kotlin(&(), env)?.auto_local(env);
        let settings = self.settings.try_into_kotlin(&(), env)?.auto_local(env);
        let base_url = self.base_url.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::StreamingServer).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                &KotlinClassName::StreamingServer_Selected.value(),
                KotlinClassName::Loadable.value(),
                KotlinClassName::Loadable.value()
            ),
            &[
                selected.as_obj().into(),
                settings.as_obj().into(),
                base_url.as_obj().into(),
            ],
        )
    }
}
