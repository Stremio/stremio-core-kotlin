use stremio_core::models::ctx::Ctx;
use stremio_core::models::player::{Player, Selected, VideoParams};
use stremio_core::types::streams::{AudioTrack, StreamItemState, SubtitleTrack};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Selected> for models::player::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            stream: self.stream.from_protobuf(),
            stream_request: self.stream_request.from_protobuf(),
            meta_request: self.meta_request.from_protobuf(),
            subtitles_path: self.subtitles_path.from_protobuf(),
        }
    }
}

impl FromProtobuf<StreamItemState> for models::player::StreamState {
    fn from_protobuf(&self) -> StreamItemState {
        StreamItemState {
            subtitle_track: self.subtitle_track.from_protobuf(),
            subtitle_delay: self.subtitle_delay.to_owned(),
            audio_track: self.audio_track.from_protobuf(),
            audio_delay: self.audio_delay.to_owned(),
            playback_speed: self.playback_speed.to_owned(),
            player_type: self.player_type.to_owned(),
        }
    }
}

impl FromProtobuf<SubtitleTrack> for models::player::SubtitleTrack {
    fn from_protobuf(&self) -> SubtitleTrack {
        SubtitleTrack {
            id: self.id.to_owned(),
            embedded: self.embedded,
            language: self.language.to_owned(),
        }
    }
}

impl FromProtobuf<AudioTrack> for models::player::AudioTrack {
    fn from_protobuf(&self) -> AudioTrack {
        AudioTrack {
            id: self.id.to_owned(),
            language: self.language.to_owned(),
        }
    }
}

impl FromProtobuf<VideoParams> for models::player::VideoParams {
    fn from_protobuf(&self) -> VideoParams {
        VideoParams {
            hash: self.hash.to_owned(),
            size: self.size.map(|x| x as u64).to_owned(),
            filename: self.filename.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::VideoParams, ()> for VideoParams {
    fn to_protobuf(&self, _args: &()) -> models::player::VideoParams {
        models::player::VideoParams {
            hash: self.hash.to_owned(),
            size: self.size.map(|x| x as i64).to_owned(),
            filename: self.filename.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::StreamState, ()> for StreamItemState {
    fn to_protobuf(&self, _args: &()) -> models::player::StreamState {
        models::player::StreamState {
            subtitle_track: self.subtitle_track.to_protobuf(&()),
            subtitle_delay: self.subtitle_delay.to_owned(),
            audio_track: self.audio_track.to_protobuf(&()),
            audio_delay: self.audio_delay.to_owned(),
            playback_speed: self.playback_speed.to_owned(),
            player_type: self.player_type.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::SubtitleTrack, ()> for SubtitleTrack {
    fn to_protobuf(&self, _args: &()) -> models::player::SubtitleTrack {
        models::player::SubtitleTrack {
            id: self.id.to_owned(),
            embedded: self.embedded,
            language: self.language.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::AudioTrack, ()> for AudioTrack {
    fn to_protobuf(&self, _args: &()) -> models::player::AudioTrack {
        models::player::AudioTrack {
            id: self.id.to_owned(),
            language: self.language.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::Selected, Ctx> for Selected {
    fn to_protobuf(&self, ctx: &Ctx) -> models::player::Selected {
        let addon_name = self.stream_request.as_ref().and_then(|request| {
            ctx.profile
                .addons
                .iter()
                .find(|addon| addon.transport_url == request.base)
                .map(|addon| &addon.manifest.name)
        });
        models::player::Selected {
            stream: self
                .stream
                .to_protobuf(&(Some(ctx), addon_name, None, None)),
            stream_request: self.stream_request.to_protobuf(&()),
            meta_request: self.meta_request.to_protobuf(&()),
            subtitles_path: self.subtitles_path.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::Player, Ctx> for Player {
    fn to_protobuf(&self, ctx: &Ctx) -> models::Player {
        models::Player {
            selected: self.selected.to_protobuf(ctx),
            video_params: self.video_params.to_protobuf(&()),
            meta_item: self.meta_item.as_ref().to_protobuf(&(
                ctx,
                self.library_item.as_ref(),
                self.watched.as_ref(),
            )),
            subtitles: self.subtitles.to_protobuf(ctx),
            next_video: self.next_video.to_protobuf(&(
                self.library_item.as_ref(),
                self.watched.as_ref(),
                None,
            )),
            series_info: self.series_info.to_protobuf(&()),
            library_item: self.library_item.to_protobuf(ctx),
            stream_state: self.stream_state.to_protobuf(&()),
        }
    }
}
