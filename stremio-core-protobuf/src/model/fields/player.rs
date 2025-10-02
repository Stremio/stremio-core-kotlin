use stremio_core::{
    deep_links::VideoDeepLinks,
    models::{
        ctx::Ctx,
        player::{Player, Selected, VideoParams},
        streaming_server::StreamingServer,
    },
    runtime::Env,
    types::{
        addon::ResourceRequest,
        player::{IntroData, IntroOutro},
        resource::Video,
        streams::{AudioTrack, StreamItemState, SubtitleTrack},
    },
};
use url::Url;

use crate::{
    bridge::{FromProtobuf, ToProtobuf},
    protobuf::stremio::core::{models, types},
};

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
            subtitle_size: self.subtitle_size.to_owned(),
            subtitle_offset: self.subtitle_offset.to_owned(),
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
            size: self.size.to_owned(),
            filename: self.filename.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::IntroOutro, ()> for IntroOutro {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> models::player::IntroOutro {
        models::player::IntroOutro {
            intro: self.intro.to_protobuf::<E>(&()),
            outro: self.outro,
        }
    }
}

impl ToProtobuf<models::player::IntroData, ()> for IntroData {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> models::player::IntroData {
        models::player::IntroData {
            from: self.from,
            to: self.to,
            duration: self.duration,
        }
    }
}

impl ToProtobuf<models::player::VideoParams, ()> for VideoParams {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::player::VideoParams {
        models::player::VideoParams {
            hash: self.hash.to_owned(),
            size: self.size.to_owned(),
            filename: self.filename.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::StreamState, ()> for StreamItemState {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::player::StreamState {
        models::player::StreamState {
            subtitle_track: self.subtitle_track.to_protobuf::<E>(&()),
            subtitle_delay: self.subtitle_delay.to_owned(),
            subtitle_size: self.subtitle_size.to_owned(),
            subtitle_offset: self.subtitle_offset.to_owned(),
            audio_track: self.audio_track.to_protobuf::<E>(&()),
            audio_delay: self.audio_delay.to_owned(),
            playback_speed: self.playback_speed.to_owned(),
            player_type: self.player_type.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::SubtitleTrack, ()> for SubtitleTrack {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::player::SubtitleTrack {
        models::player::SubtitleTrack {
            id: self.id.to_owned(),
            embedded: self.embedded,
            language: self.language.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::AudioTrack, ()> for AudioTrack {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::player::AudioTrack {
        models::player::AudioTrack {
            id: self.id.to_owned(),
            language: self.language.to_owned(),
        }
    }
}

impl ToProtobuf<models::player::Selected, (&Ctx, Option<&Url>)> for Selected {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url): &(&Ctx, Option<&Url>),
    ) -> models::player::Selected {
        let addon_name = self.stream_request.as_ref().and_then(|request| {
            ctx.profile
                .addons
                .iter()
                .find(|addon| addon.transport_url == request.base)
                .map(|addon| &addon.manifest.name)
        });
        models::player::Selected {
            stream: self.stream.to_protobuf::<E>(&(
                Some(*ctx),
                *streaming_server_url,
                addon_name,
                None,
                None,
            )),
            stream_request: self.stream_request.to_protobuf::<E>(&()),
            meta_request: self.meta_request.to_protobuf::<E>(&()),
            subtitles_path: self.subtitles_path.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<models::Player, (&Ctx, &StreamingServer)> for Player {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server): &(&Ctx, &StreamingServer),
    ) -> models::Player {
        models::Player {
            selected: self
                .selected
                .to_protobuf::<E>(&(*ctx, streaming_server.base_url.as_ref())),
            video_params: self.video_params.to_protobuf::<E>(&()),
            meta_item: self.meta_item.as_ref().to_protobuf::<E>(&(
                *ctx,
                streaming_server.base_url.as_ref(),
                self.library_item.as_ref(),
                self.watched.as_ref(),
            )),
            subtitles: self.subtitles.to_protobuf::<E>(*ctx),
            next_video: self
                .selected
                .as_ref()
                .and_then(|selected| {
                    Some((
                        selected.meta_request.as_ref()?,
                        selected.stream_request.as_ref()?,
                    ))
                })
                .and_then(|(meta_request, stream_request)| {
                    self.next_video.to_protobuf::<E>(&(
                        *ctx,
                        streaming_server.base_url.as_ref(),
                        meta_request,
                        stream_request,
                    ))
                }),
            series_info: self.series_info.to_protobuf::<E>(&()),
            library_item: self.library_item.to_protobuf::<E>(&(*ctx, None)),
            stream_state: self.stream_state.to_protobuf::<E>(&()),
            intro_outro: self.intro_outro.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<types::Video, (&Ctx, Option<&Url>, &ResourceRequest, &ResourceRequest)> for Video {
    fn to_protobuf<E: Env + 'static>(
        &self,
        (ctx, streaming_server_url, meta_request, stream_request): &(
            &Ctx,
            Option<&Url>,
            &ResourceRequest,
            &ResourceRequest,
        ),
    ) -> types::Video {
        types::Video {
            id: self.id.to_string(),
            title: self.title.to_string(),
            released: self.released.to_protobuf::<E>(&()),
            overview: self.overview.clone(),
            thumbnail: self.thumbnail.clone(),
            series_info: self.series_info.to_protobuf::<E>(&()),
            deep_links: VideoDeepLinks::from((
                self,
                *stream_request,
                *meta_request,
                &streaming_server_url.cloned(),
                &ctx.profile.settings,
            ))
            .to_protobuf::<E>(&()),
            ..Default::default()
        }
    }
}
