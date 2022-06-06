use std::cmp;
use std::convert::TryFrom;

use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use stremio_core::types::profile::{Auth, GDPRConsent, Profile, Settings, User};
use url::Url;

use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for GDPRConsent {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let tos = env.call_method(value, "getTos", "()Z", &[])?.z()?;
        let privacy = env.call_method(value, "getPrivacy", "()Z", &[])?.z()?;
        let marketing = env.call_method(value, "getMarketing", "()Z", &[])?.z()?;
        Ok(GDPRConsent {
            tos,
            privacy,
            marketing,
        })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for GDPRConsent {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        env.new_object(
            classes.get(&KotlinClassName::GDPRConsent).unwrap(),
            "(ZZZ)V",
            &[self.tos.into(), self.privacy.into(), self.marketing.into()],
        )
    }
}

impl TryFromKotlin for Settings {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let interface_language = env
            .call_method(
                value,
                "getInterfaceLanguage",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let interface_language = String::try_from_kotlin(interface_language.as_obj(), env)?;
        let streaming_server_url = env
            .call_method(
                value,
                "getStreamingServerUrl",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let streaming_server_url = String::try_from_kotlin(streaming_server_url.as_obj(), env)?;
        let streaming_server_url =
            Url::parse(&streaming_server_url).expect("Settings.streaming_server_url parse failed");
        let binge_watching = env
            .call_method(value, "getBingeWatching", "()Z", &[])?
            .z()?;
        let play_in_background = env
            .call_method(value, "getPlayInBackground", "()Z", &[])?
            .z()?;
        let play_in_external_player = env
            .call_method(value, "getPlayInExternalPlayer", "()Z", &[])?
            .z()?;
        let hardware_decoding = env
            .call_method(value, "getHardwareDecoding", "()Z", &[])?
            .z()?;
        let subtitles_language = env
            .call_method(
                value,
                "getSubtitlesLanguage",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let subtitles_language = String::try_from_kotlin(subtitles_language.as_obj(), env)?;
        let subtitles_size = env
            .call_method(value, "getSubtitlesSize", "()I", &[])?
            .i()?;
        let subtitles_size = u8::try_from(cmp::max(subtitles_size, 0)).unwrap_or(u8::MAX);
        let subtitles_font = env
            .call_method(
                value,
                "getSubtitlesFont",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let subtitles_font = String::try_from_kotlin(subtitles_font.as_obj(), env)?;
        let subtitles_bold = env
            .call_method(value, "getSubtitlesBold", "()Z", &[])?
            .z()?;
        let subtitles_offset = env
            .call_method(value, "getSubtitlesOffset", "()I", &[])?
            .i()?;
        let subtitles_offset = u8::try_from(cmp::max(subtitles_offset, 0)).unwrap_or(u8::MAX);
        let subtitles_text_color = env
            .call_method(
                value,
                "getSubtitlesTextColor",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let subtitles_text_color = String::try_from_kotlin(subtitles_text_color.as_obj(), env)?;
        let subtitles_background_color = env
            .call_method(
                value,
                "getSubtitlesBackgroundColor",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let subtitles_background_color =
            String::try_from_kotlin(subtitles_background_color.as_obj(), env)?;
        let subtitles_outline_color = env
            .call_method(
                value,
                "getSubtitlesOutlineColor",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let subtitles_outline_color =
            String::try_from_kotlin(subtitles_outline_color.as_obj(), env)?;
        let seek_time_duration = env
            .call_method(value, "getSeekTimeDuration", "()J", &[])?
            .j()?;
        let seek_time_duration = u32::try_from(cmp::max(seek_time_duration, 0)).unwrap_or(u32::MAX);
        Ok(Settings {
            interface_language,
            streaming_server_url,
            binge_watching,
            play_in_background,
            play_in_external_player,
            hardware_decoding,
            subtitles_language,
            subtitles_size,
            subtitles_font,
            subtitles_bold,
            subtitles_offset,
            subtitles_text_color,
            subtitles_background_color,
            subtitles_outline_color,
            seek_time_duration,
            ..Default::default()
        })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Settings {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let interface_language = self
            .interface_language
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let streaming_server_url = self
            .streaming_server_url
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let binge_watching = self.binge_watching.into();
        let play_in_background = self.play_in_background.into();
        let play_in_external_player = self.play_in_external_player.into();
        let hardware_decoding = self.hardware_decoding.into();
        let subtitles_language = self
            .subtitles_language
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let subtitles_size = (self.subtitles_size as i32).into();
        let subtitles_font = self
            .subtitles_font
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let subtitles_bold = self.subtitles_bold.into();
        let subtitles_offset = (self.subtitles_offset as i32).into();
        let subtitles_text_color = self
            .subtitles_text_color
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let subtitles_background_color = self
            .subtitles_background_color
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let subtitles_outline_color = self
            .subtitles_outline_color
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let seek_time_duration = (self.seek_time_duration as i64).into();
        let unknown_fields = env.new_object(
            classes.get(&KotlinClassName::HashMap).unwrap(),
            "()V",
            &[],
        )?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Profile_Settings).unwrap(),
            format!(
                "(L{};L{};ZZZZL{};IL{};ZIL{};L{};L{};JL{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::Map.value(),
            ),
            &[
                interface_language.as_obj().into(),
                streaming_server_url.as_obj().into(),
                binge_watching,
                play_in_background,
                play_in_external_player,
                hardware_decoding,
                subtitles_language.as_obj().into(),
                subtitles_size,
                subtitles_font.as_obj().into(),
                subtitles_bold,
                subtitles_offset,
                subtitles_text_color.as_obj().into(),
                subtitles_background_color.as_obj().into(),
                subtitles_outline_color.as_obj().into(),
                seek_time_duration,
                unknown_fields.as_obj().into(),
            ],
        )
    }
}

impl ToProtobuf<types::LinkAuthKey, ()> for LinkAuthKey {
    fn to_protobuf(&self, _args: &()) -> types::LinkAuthKey {
        types::LinkAuthKey {
            auth_key: self.auth_key.to_string(),
        }
    }
}

impl ToProtobuf<types::LinkCodeResponse, ()> for LinkCodeResponse {
    fn to_protobuf(&self, _args: &()) -> types::LinkCodeResponse {
        types::LinkCodeResponse {
            code: self.code.to_string(),
            link: self.link.to_string(),
            qrcode: self.qrcode.to_string(),
        }
    }
}

impl ToProtobuf<types::GdprConsent, ()> for GDPRConsent {
    fn to_protobuf(&self, _args: &()) -> types::GdprConsent {
        types::GdprConsent {
            tos: self.tos,
            privacy: self.privacy,
            marketing: self.marketing,
        }
    }
}

impl ToProtobuf<types::User, ()> for User {
    fn to_protobuf(&self, _args: &()) -> types::User {
        types::User {
            id: self.id.to_string(),
            email: self.email.to_string(),
            fb_id: self.fb_id.clone(),
            avatar: self.avatar.clone(),
            gdpr_consent: self.gdpr_consent.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<types::Auth, ()> for Auth {
    fn to_protobuf(&self, _args: &()) -> types::Auth {
        types::Auth {
            key: self.key.0.to_string(),
            user: self.user.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<types::profile::Settings, ()> for Settings {
    fn to_protobuf(&self, _args: &()) -> types::profile::Settings {
        types::profile::Settings {
            interface_language: self.interface_language.to_string(),
            streaming_server_url: self.streaming_server_url.to_string(),
            binge_watching: self.binge_watching,
            play_in_background: self.play_in_background,
            play_in_external_player: self.play_in_external_player,
            hardware_decoding: self.hardware_decoding,
            subtitles_language: self.subtitles_language.to_string(),
            subtitles_size: self.subtitles_size as i32,
            subtitles_font: self.subtitles_font.to_string(),
            subtitles_bold: self.subtitles_bold,
            subtitles_offset: self.subtitles_offset as i32,
            subtitles_text_color: self.subtitles_text_color.to_string(),
            subtitles_background_color: self.subtitles_background_color.to_string(),
            subtitles_outline_color: self.subtitles_outline_color.to_string(),
            seek_time_duration: self.seek_time_duration as i64,
        }
    }
}

impl ToProtobuf<types::Profile, ()> for Profile {
    fn to_protobuf(&self, _args: &()) -> types::Profile {
        types::Profile {
            auth: self.auth.to_protobuf(&()),
            settings: self.settings.to_protobuf(&()),
        }
    }
}
