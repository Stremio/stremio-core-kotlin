use std::cmp;
use std::convert::TryFrom;

use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use stremio_core::types::profile::{Auth, GDPRConsent, Profile, Settings, User};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<GDPRConsent> for types::GdprConsent {
    fn from_protobuf(&self) -> GDPRConsent {
        GDPRConsent {
            tos: self.tos,
            privacy: self.privacy,
            marketing: self.marketing,
            from: self.from.clone(),
        }
    }
}

impl FromProtobuf<Settings> for types::profile::Settings {
    fn from_protobuf(&self) -> Settings {
        Settings {
            interface_language: self.interface_language.to_string(),
            streaming_server_url: self.streaming_server_url.from_protobuf(),
            binge_watching: self.binge_watching,
            play_in_background: self.play_in_background,
            play_in_external_player: self.play_in_external_player,
            hardware_decoding: self.hardware_decoding,
            audio_passthrough: self.audio_passthrough,
            audio_language: self.audio_language.to_string(),
            secondary_audio_language: self.secondary_audio_language.clone(),
            subtitles_language: self.subtitles_language.to_string(),
            secondary_subtitles_language: self.secondary_subtitles_language.clone(),
            subtitles_size: u8::try_from(cmp::max(self.subtitles_size, 0)).unwrap_or(u8::MAX),
            subtitles_font: self.subtitles_font.to_string(),
            subtitles_bold: self.subtitles_bold,
            subtitles_offset: u8::try_from(cmp::max(self.subtitles_offset, 0)).unwrap_or(u8::MAX),
            subtitles_text_color: self.subtitles_text_color.to_string(),
            subtitles_background_color: self.subtitles_background_color.to_string(),
            subtitles_outline_color: self.subtitles_outline_color.to_string(),
            seek_time_duration: u32::try_from(cmp::max(self.seek_time_duration, 0))
                .unwrap_or(u32::MAX),
            streaming_server_warning_dismissed: None,
        }
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
            from: self.from.clone(),
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
            audio_passthrough: self.audio_passthrough,
            audio_language: self.audio_language.to_string(),
            subtitles_language: self.subtitles_language.to_string(),
            subtitles_size: self.subtitles_size as i32,
            subtitles_font: self.subtitles_font.to_string(),
            subtitles_bold: self.subtitles_bold,
            subtitles_offset: self.subtitles_offset as i32,
            subtitles_text_color: self.subtitles_text_color.to_string(),
            subtitles_background_color: self.subtitles_background_color.to_string(),
            subtitles_outline_color: self.subtitles_outline_color.to_string(),
            seek_time_duration: self.seek_time_duration as i64,
            secondary_audio_language: self.secondary_audio_language.clone(),
            secondary_subtitles_language: self.secondary_subtitles_language.clone(),
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
