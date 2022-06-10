use boolinator::Boolinator;
use stremio_core::deep_links::MetaItemDeepLinks;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::meta_details::{MetaDetails, Selected};
use stremio_core::runtime::Env;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::{MetaItem, SeriesInfo, Video};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::env::AndroidEnv;
use crate::protobuf::stremio::core::{models, types};

impl FromProtobuf<Selected> for models::meta_details::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            meta_path: self.meta_path.from_protobuf(),
            stream_path: self.stream_path.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::meta_details::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::meta_details::Selected {
        models::meta_details::Selected {
            meta_path: self.meta_path.to_protobuf(&()),
            stream_path: self.stream_path.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<types::video::SeriesInfo, ()> for SeriesInfo {
    fn to_protobuf(&self, _args: &()) -> types::video::SeriesInfo {
        types::video::SeriesInfo {
            season: self.season as i64,
            episode: self.episode as i64,
        }
    }
}

impl ToProtobuf<types::Video, (Option<&MetaDetails>, Option<String>)> for Video {
    fn to_protobuf(
        &self,
        (details, addon_name): &(Option<&MetaDetails>, Option<String>),
    ) -> types::Video {
        types::Video {
            id: self.id.to_string(),
            title: self.title.to_string(),
            released: self.released.to_protobuf(&()),
            overview: self.overview.clone(),
            thumbnail: self.thumbnail.clone(),
            streams: self
                .streams
                .to_protobuf(&(addon_name.to_owned(), None, None)),
            series_info: self.series_info.to_protobuf(&()),
            upcoming: self
                .released
                .map(|released| released > AndroidEnv::now())
                .unwrap_or_default(),
            watched: details
                .and_then(|details| details.watched.to_owned())
                .map(|watched| watched.get_video(&self.id))
                .unwrap_or_default(),
            current_video: details
                .and_then(|details| details.library_item.to_owned())
                .and_then(|library_item| library_item.state.video_id)
                .map(|current_video_id| current_video_id == self.id)
                .unwrap_or_default(),
        }
    }
}

impl ToProtobuf<types::MetaItem, (Option<&MetaDetails>, Option<String>, ResourceRequest)>
    for MetaItem
{
    fn to_protobuf(
        &self,
        (details, addon_name, meta_request): &(
            Option<&MetaDetails>,
            Option<String>,
            ResourceRequest,
        ),
    ) -> types::MetaItem {
        types::MetaItem {
            id: self.preview.id.to_string(),
            r#type: self.preview.r#type.to_string(),
            name: self.preview.name.to_string(),
            poster_shape: self.preview.poster_shape.to_protobuf(&()) as i32,
            poster: self.preview.poster.clone(),
            background: self.preview.background.clone(),
            logo: self.preview.logo.clone(),
            description: self.preview.description.clone(),
            release_info: self.preview.release_info.clone(),
            runtime: self.preview.runtime.clone(),
            released: self.preview.released.to_protobuf(&()),
            links: self.preview.links.to_protobuf(&()),
            trailer_streams: self
                .preview
                .trailer_streams
                .to_protobuf(&(None, None, None)),
            videos: self.videos.to_protobuf(&(*details, addon_name.to_owned())),
            behavior_hints: self.preview.behavior_hints.to_protobuf(&()),
            deep_links: MetaItemDeepLinks::from((self, meta_request)).to_protobuf(&()),
            progress: details
                .and_then(|details| details.library_item.to_owned())
                .and_then(|item| {
                    if item.state.time_offset > 0 && item.state.duration > 0 {
                        Some(item.state.time_offset as f64 / item.state.duration as f64)
                    } else {
                        None
                    }
                }),
            watched: details
                .and_then(|details| details.watched.to_owned())
                .map(|watched| {
                    watched.get_video(
                        &self
                            .preview
                            .behavior_hints
                            .default_video_id
                            .to_owned()
                            .unwrap_or(self.preview.id.to_owned()),
                    )
                })
                .unwrap_or_default(),
            in_library: details
                .and_then(|details| details.library_item.to_owned())
                .map(|item| !item.removed)
                .unwrap_or_default(),
        }
    }
}

impl ToProtobuf<models::MetaDetails, Ctx> for MetaDetails {
    fn to_protobuf(&self, ctx: &Ctx) -> models::MetaDetails {
        let meta_item = self
            .meta_items
            .iter()
            .find(|meta_item| meta_item.content.as_ref().map_or(false, |x| x.is_ready()))
            .or_else(|| {
                if self
                    .meta_items
                    .iter()
                    .all(|meta_item| meta_item.content.as_ref().map_or(false, |x| x.is_err()))
                {
                    self.meta_items.first()
                } else {
                    self.meta_items
                        .iter()
                        .find(|catalog| catalog.content.as_ref().map_or(false, |x| x.is_loading()))
                }
            });
        let meta_request = meta_item.map(|item| &item.request);
        let title = meta_item
            .and_then(|meta_item| meta_item.content.to_owned())
            .and_then(|meta_item| meta_item.ready())
            .map(|meta_item| {
                meta_item
                    .preview
                    .behavior_hints
                    .default_video_id
                    .is_none()
                    .as_option()
                    .and_then(|_| self.selected.as_ref())
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
            selected: self.selected.to_protobuf(&()),
            title,
            meta_item: meta_item.to_protobuf(&(ctx, Some(self))),
            streams: self.streams.to_protobuf(&(ctx, meta_request)),
        }
    }
}
