use std::cmp;
use std::convert::TryFrom;

use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use stremio_core::types::profile::{
    Auth, FrameRateMatchingStrategy, GDPRConsent, Profile, Settings, TraktInfo, User,
};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<FrameRateMatchingStrategy> for types::profile::FrameRateMatchingStrategy {
    fn from_protobuf(&self) -> FrameRateMatchingStrategy {
        match self {
            types::profile::FrameRateMatchingStrategy::Disabled => {
                FrameRateMatchingStrategy::Disabled
            }
            types::profile::FrameRateMatchingStrategy::FrameRateOnly => {
                FrameRateMatchingStrategy::FrameRateOnly
            }
            types::profile::FrameRateMatchingStrategy::FrameRateAndResolution => {
                FrameRateMatchingStrategy::FrameRateAndResolution
            }
        }
    }
}

impl ToProtobuf<types::profile::FrameRateMatchingStrategy, ()> for FrameRateMatchingStrategy {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::profile::FrameRateMatchingStrategy {
        match self {
            FrameRateMatchingStrategy::Disabled => {
                types::profile::FrameRateMatchingStrategy::Disabled
            }
            FrameRateMatchingStrategy::FrameRateOnly => {
                types::profile::FrameRateMatchingStrategy::FrameRateOnly
            }
            FrameRateMatchingStrategy::FrameRateAndResolution => {
                types::profile::FrameRateMatchingStrategy::FrameRateAndResolution
            }
        }
    }
}

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
            hide_spoilers: self.hide_spoilers,
            streaming_server_url: self.streaming_server_url.from_protobuf(),
            player_type: self.player_type.clone(),
            binge_watching: self.binge_watching,
            play_in_background: self.play_in_background,
            hardware_decoding: self.hardware_decoding,
            video_mode: self.video_mode.to_owned(),
            frame_rate_matching_strategy: types::profile::FrameRateMatchingStrategy::try_from(
                self.frame_rate_matching_strategy,
            )
            .ok()
            .from_protobuf()
            .unwrap_or(FrameRateMatchingStrategy::Disabled),
            next_video_notification_duration: u32::try_from(cmp::max(
                self.next_video_notification_duration,
                0,
            ))
            .unwrap_or(u32::MAX),
            audio_passthrough: self.audio_passthrough,
            audio_language: self.audio_language.clone(),
            secondary_audio_language: self.secondary_audio_language.clone(),
            subtitles_language: self.subtitles_language.clone(),
            secondary_subtitles_language: self.secondary_subtitles_language.clone(),
            subtitles_size: u8::try_from(cmp::max(self.subtitles_size, 0)).unwrap_or(u8::MAX),
            subtitles_font: self.subtitles_font.to_string(),
            subtitles_bold: self.subtitles_bold,
            subtitles_offset: u8::try_from(cmp::max(self.subtitles_offset, 0)).unwrap_or(u8::MAX),
            subtitles_text_color: self.subtitles_text_color.to_string(),
            subtitles_background_color: self.subtitles_background_color.to_string(),
            subtitles_outline_color: self.subtitles_outline_color.to_string(),
            subtitles_opacity: u8::try_from(cmp::max(self.subtitles_opacity, 0)).unwrap_or(u8::MAX),
            esc_exit_fullscreen: self.esc_exit_fullscreen,
            seek_time_duration: u32::try_from(cmp::max(self.seek_time_duration, 0))
                .unwrap_or(u32::MAX),
            seek_short_time_duration: u32::try_from(cmp::max(self.seek_time_duration, 0))
                .unwrap_or(u32::MAX),
            pause_on_minimize: self.pause_on_minimize,
            surround_sound: self.surround_sound,
            streaming_server_warning_dismissed: None,
            server_in_foreground: self.server_in_foreground,
            send_crash_reports: self.send_crash_reports,
            quit_on_close: self.quit_on_close,
            gamepad_support: self.gamepad_support,
        }
    }
}

impl ToProtobuf<types::LinkAuthKey, ()> for LinkAuthKey {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::LinkAuthKey {
        types::LinkAuthKey {
            auth_key: self.auth_key.to_string(),
        }
    }
}

impl ToProtobuf<types::LinkCodeResponse, ()> for LinkCodeResponse {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::LinkCodeResponse {
        types::LinkCodeResponse {
            code: self.code.to_string(),
            link: self.link.to_string(),
            qrcode: self.qrcode.to_string(),
        }
    }
}

impl ToProtobuf<types::GdprConsent, ()> for GDPRConsent {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::GdprConsent {
        types::GdprConsent {
            tos: self.tos,
            privacy: self.privacy,
            marketing: self.marketing,
            from: self.from.clone(),
        }
    }
}

impl ToProtobuf<types::TraktInfo, ()> for TraktInfo {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::TraktInfo {
        types::TraktInfo {
            created_at: self.created_at.to_protobuf::<E>(&()),
            expires_in: self.expires_in.to_protobuf::<E>(&()),
            access_token: self.access_token.clone(),
        }
    }
}

impl ToProtobuf<types::User, ()> for User {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::User {
        types::User {
            id: self.id.to_string(),
            email: self.email.to_string(),
            fb_id: self.fb_id.clone(),
            avatar: self.avatar.clone(),
            gdpr_consent: self.gdpr_consent.to_protobuf::<E>(&()),
            date_registered: self.date_registered.to_protobuf::<E>(&()),
            last_modified: self.last_modified.to_protobuf::<E>(&()),
            premium_expire: self.premium_expire.to_protobuf::<E>(&()),
            trakt: self.trakt.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<types::Auth, ()> for Auth {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::Auth {
        types::Auth {
            key: self.key.0.to_string(),
            user: self.user.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<types::profile::Settings, ()> for Settings {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::profile::Settings {
        types::profile::Settings {
            interface_language: self.interface_language.to_string(),
            hide_spoilers: self.hide_spoilers,
            streaming_server_url: self.streaming_server_url.to_string(),
            binge_watching: self.binge_watching,
            play_in_background: self.play_in_background,
            hardware_decoding: self.hardware_decoding,
            video_mode: self.video_mode.to_owned(),
            audio_passthrough: self.audio_passthrough,
            audio_language: self.audio_language.clone(),
            subtitles_language: self.subtitles_language.clone(),
            subtitles_size: self.subtitles_size as i32,
            subtitles_font: self.subtitles_font.to_string(),
            subtitles_bold: self.subtitles_bold,
            subtitles_offset: self.subtitles_offset as i32,
            subtitles_text_color: self.subtitles_text_color.to_string(),
            subtitles_background_color: self.subtitles_background_color.to_string(),
            subtitles_outline_color: self.subtitles_outline_color.to_string(),
            subtitles_opacity: self.subtitles_opacity as i32,
            esc_exit_fullscreen: self.esc_exit_fullscreen,
            seek_time_duration: self.seek_time_duration as i64,
            seek_short_time_duration: self.seek_short_time_duration as i64,
            pause_on_minimize: self.pause_on_minimize,
            secondary_audio_language: self.secondary_audio_language.clone(),
            secondary_subtitles_language: self.secondary_subtitles_language.clone(),
            player_type: self.player_type.clone(),
            frame_rate_matching_strategy: self.frame_rate_matching_strategy.to_protobuf::<E>(&())
                as i32,
            next_video_notification_duration: self.next_video_notification_duration as i64,
            surround_sound: self.surround_sound,
            server_in_foreground: self.server_in_foreground,
            send_crash_reports: self.send_crash_reports,
            quit_on_close: self.quit_on_close,
            gamepad_support: self.gamepad_support,
        }
    }
}

impl ToProtobuf<types::Profile, ()> for Profile {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::Profile {
        types::Profile {
            auth: self.auth.to_protobuf::<E>(&()),
            settings: self.settings.to_protobuf::<E>(&()),
        }
    }
}
