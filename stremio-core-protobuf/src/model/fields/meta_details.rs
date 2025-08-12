use boolinator::Boolinator;

use stremio_core::{
    deep_links::{MetaItemDeepLinks, VideoDeepLinks},
    models::{
        common::Loadable,
        ctx::Ctx,
        meta_details::{MetaDetails, Selected},
        streaming_server::StreamingServer,
    },
    runtime::{Env, EnvError},
    types::{
        addon::ResourceRequest,
        library::LibraryItem,
        rating::{Rating, RatingInfo},
        resource::{MetaItem, SeriesInfo, Video},
        watched_bitfield::WatchedBitField,
    },
};

use url::Url;

use crate::{
    bridge::{FromProtobuf, ToProtobuf},
    protobuf::stremio::core::{models, types},
};

impl FromProtobuf<Selected> for models::meta_details::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            meta_path: self.meta_path.from_protobuf(),
            stream_path: self.stream_path.from_protobuf(),
            guess_stream: self.guess_stream_path,
        }
    }
}

impl ToProtobuf<models::meta_details::Selected, ()> for Selected {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> models::meta_details::Selected {
        models::meta_details::Selected {
            meta_path: self.meta_path.to_protobuf::<E>(&()),
            stream_path: self.stream_path.to_protobuf::<E>(&()),
            guess_stream_path: self.guess_stream,
        }
    }
}

impl ToProtobuf<types::video::SeriesInfo, ()> for SeriesInfo {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::video::SeriesInfo {
        types::video::SeriesInfo {
            season: self.season as i64,
            episode: self.episode as i64,
        }
    }
}

impl FromProtobuf<SeriesInfo> for types::video::SeriesInfo {
    fn from_protobuf(&self) -> SeriesInfo {
        SeriesInfo {
            season: self.season.unsigned_abs() as u32,
            episode: self.episode.unsigned_abs() as u32,
        }
    }
}

impl
    ToProtobuf<
        types::Video,
        (
            &Ctx,
            Option<&Url>,
            // &StreamingServer,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    > for Video
{
    fn to_protobuf<E: Env + 'static>(
        &self,
        (ctx, streaming_server_url, library_item, watched, addon_name, request): &(
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    ) -> types::Video {
        types::Video {
            id: self.id.to_string(),
            title: self.title.to_string(),
            released: self.released.to_protobuf::<E>(&()),
            overview: self.overview.clone(),
            thumbnail: self.thumbnail.clone(),
            streams: self.streams.to_protobuf::<E>(&(
                None,
                *streaming_server_url,
                *addon_name,
                None,
                None,
            )),
            series_info: self.series_info.to_protobuf::<E>(&()),
            upcoming: self
                .released
                .map(|released| released > E::now())
                .unwrap_or_default(),
            watched: watched
                .map(|watched| watched.get_video(&self.id))
                .unwrap_or_default(),
            current_video: library_item
                .and_then(|library_item| library_item.state.video_id.to_owned())
                .map(|current_video_id| current_video_id == self.id)
                .unwrap_or_default(),
            progress: library_item.and_then(|library_item| {
                ctx.library
                    .items
                    .get(&library_item.id)
                    .filter(|library_item| Some(self.id.to_owned()) == library_item.state.video_id)
                    .map(|library_item| library_item.progress())
            }),
            // scheduled: meta_item.preview.behavior_hints.has_scheduled_videos,
            deep_links: VideoDeepLinks::from((
                self,
                *request,
                // necessary because Core doesn't use the
                // preferred `Option<&Url>` format
                &streaming_server_url.map(Clone::clone),
                &ctx.profile.settings,
            ))
            .to_protobuf::<E>(&()),
        }
    }
}

impl FromProtobuf<Video> for types::Video {
    fn from_protobuf(&self) -> Video {
        Video {
            id: self.id.to_owned(),
            title: self.title.to_owned(),
            released: self.released.to_owned().from_protobuf(),
            overview: self.overview.to_owned(),
            thumbnail: self.thumbnail.to_owned(),
            streams: self.streams.to_owned().from_protobuf(),
            series_info: self.series_info.to_owned().from_protobuf(),
            // trailer_streams: self.trailer_streams.to_owned(),
            // TODO: implement trailer streams!
            trailer_streams: vec![],
        }
    }
}

impl ToProtobuf<types::VideoDeepLinks, ()> for VideoDeepLinks {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::VideoDeepLinks {
        types::VideoDeepLinks {
            meta_details_videos: self.meta_details_videos.clone(),
            meta_details_streams: self.meta_details_streams.clone(),
            player: self.player.clone(),
            external_player: types::video_deep_links::ExternalPlayerLink {
                download: self
                    .external_player
                    .as_ref()
                    .and_then(|ep| ep.download.clone()),
                streaming: self
                    .external_player
                    .as_ref()
                    .and_then(|ep| ep.streaming.clone()),
                open_player: self.external_player.as_ref().and_then(|ep| {
                    ep.open_player
                        .as_ref()
                        .map(|core_op| types::video_deep_links::OpenPlayerLink {
                            ios: core_op.ios.clone(),
                            macos: core_op.macos.clone(),
                            visionos: core_op.visionos.clone(),
                        })
                }),
            },
        }
    }
}

impl
    ToProtobuf<
        types::MetaItem,
        (
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    > for MetaItem
{
    fn to_protobuf<E: Env + 'static>(
        &self,
        (ctx, streaming_server_url, library_item, watched, addon_name, meta_request): &(
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    ) -> types::MetaItem {
        types::MetaItem {
            id: self.preview.id.to_string(),
            r#type: self.preview.r#type.to_string(),
            name: self.preview.name.to_string(),
            poster_shape: self.preview.poster_shape.to_protobuf::<E>(&()) as i32,
            poster: self.preview.poster.to_protobuf::<E>(&()),
            background: self.preview.background.to_protobuf::<E>(&()),
            logo: self.preview.logo.to_protobuf::<E>(&()),
            description: self.preview.description.clone(),
            release_info: self.preview.release_info.clone(),
            runtime: self.preview.runtime.clone(),
            released: self.preview.released.to_protobuf::<E>(&()),
            links: self.preview.links.to_protobuf::<E>(&()),
            trailer_streams: self
                .preview
                .trailer_streams
                .to_protobuf::<E>(&(None, None, None, None, None)),
            videos: self.videos.to_protobuf::<E>(&(
                *ctx,
                *streaming_server_url,
                *library_item,
                *watched,
                *addon_name,
                *meta_request,
            )),

            behavior_hints: self.preview.behavior_hints.to_protobuf::<E>(&()),
            deep_links: MetaItemDeepLinks::from((self, *meta_request)).to_protobuf::<E>(&()),
            progress: library_item.and_then(|item| {
                if item.state.time_offset > 0 && item.state.duration > 0 {
                    Some(item.state.time_offset as f64 / item.state.duration as f64)
                } else {
                    None
                }
            }),
            in_library: library_item.map(|item| !item.removed).unwrap_or_default(),
            watched: library_item.map(|item| item.watched()).unwrap_or_default(),
            receive_notifications: library_item
                .map(|item| !item.state.no_notif)
                .unwrap_or_default(),
        }
    }
}

impl ToProtobuf<types::Rating, ()> for Rating {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::Rating {
        match self {
            Rating::Watched => types::Rating::Watched,
            Rating::Liked => types::Rating::Liked,
            Rating::Loved => types::Rating::Loved,
        }
    }
}

impl FromProtobuf<Rating> for types::Rating {
    fn from_protobuf(&self) -> Rating {
        match self {
            types::Rating::Watched => Rating::Watched,
            types::Rating::Liked => Rating::Liked,
            types::Rating::Loved => Rating::Loved,
        }
    }
}

impl ToProtobuf<types::RatingInfo, ()> for RatingInfo {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::RatingInfo {
        types::RatingInfo {
            meta_id: self.meta_id.clone(),
            status: self.status.as_ref().map(|s| s.to_protobuf::<E>(&()) as i32),
        }
    }
}

impl FromProtobuf<RatingInfo> for types::RatingInfo {
    fn from_protobuf(&self) -> RatingInfo {
        RatingInfo {
            meta_id: self.meta_id.clone(),
            status: self
                .status
                .and_then(|s| types::Rating::try_from(s).ok())
                .map(|s| s.from_protobuf()),
        }
    }
}

impl ToProtobuf<models::MetaDetails, (&Ctx, &StreamingServer)> for MetaDetails {
    fn to_protobuf<E: Env + 'static>(
        &self,
        (ctx, streaming_server): &(&Ctx, &StreamingServer),
    ) -> models::MetaDetails {
        let meta_item = self
            .meta_items
            .iter()
            .find(|meta_item| meta_item.content.as_ref().is_some_and(|x| x.is_ready()))
            .or_else(|| {
                if self
                    .meta_items
                    .iter()
                    .all(|meta_item| meta_item.content.as_ref().is_some_and(|x| x.is_err()))
                {
                    self.meta_items.first()
                } else {
                    self.meta_items
                        .iter()
                        .find(|catalog| catalog.content.as_ref().is_some_and(|x| x.is_loading()))
                }
            });
        let streams = if self.meta_streams.is_empty() {
            &self.streams
        } else {
            &self.meta_streams
        };
        let meta_request = meta_item.map(|item| &item.request);
        let title = meta_item
            .and_then(|meta_item| meta_item.content.as_ref())
            .and_then(|meta_item| meta_item.ready())
            .map(|meta_item| {
                meta_item
                    .preview
                    .behavior_hints
                    .default_video_id
                    .is_none()
                    .as_option()
                    .and(self.selected.as_ref())
                    .and_then(|selected| selected.stream_path.as_ref())
                    .and_then(|stream_path| {
                        meta_item
                            .videos
                            .iter()
                            .find(|video| video.id == stream_path.id)
                    })
                    .map(|video| match &video.series_info {
                        Some(series_info) => format!(
                            "{} - {} ({}x{})",
                            &meta_item.preview.name,
                            &video.title,
                            &series_info.season,
                            &series_info.episode
                        ),
                        _ => format!("{} - {}", &meta_item.preview.name, &video.title),
                    })
                    .unwrap_or_else(|| meta_item.preview.name.to_owned())
            });
        models::MetaDetails {
            selected: self.selected.to_protobuf::<E>(&()),
            title,
            meta_item: meta_item.to_protobuf::<E>(&(
                *ctx,
                streaming_server.base_url.as_ref(),
                self.library_item.as_ref(),
                self.watched.as_ref(),
            )),
            streams: streams.to_protobuf::<E>(&(
                ctx,
                streaming_server.base_url.as_ref(),
                meta_request,
            )),
            last_used_stream: self.last_used_stream.to_protobuf::<E>(&(
                *ctx,
                streaming_server.base_url.as_ref(),
                meta_request,
            )),
            rating_info: Some(self.rating_info.to_protobuf::<E>(&())),
        }
    }
}

impl ToProtobuf<models::LoadableRatingInfo, ()> for Option<Loadable<RatingInfo, EnvError>> {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> models::LoadableRatingInfo {
        use models::loadable_rating_info::Content;

        let content = match self {
            Some(loadable) => match loadable {
                Loadable::Loading => Some(Content::Loading(models::Loading {})),
                Loadable::Err(error) => Some(Content::Error(models::Error {
                    message: error.to_string(),
                })),
                Loadable::Ready(rating_info) => {
                    Some(Content::Ready(rating_info.to_protobuf::<E>(&())))
                }
            },
            None => None,
        };

        models::LoadableRatingInfo { content }
    }
}
