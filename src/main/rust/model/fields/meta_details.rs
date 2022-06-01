use boolinator::Boolinator;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::meta_details::{MetaDetails, Selected};
use stremio_core::types::addon::ResourcePath;
use stremio_core::types::resource::{MetaItem, SeriesInfo, Video};
use stremio_deeplinks::MetaItemDeepLinks;

use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::{models, types};

impl TryFromKotlin for Selected {
    fn try_from_kotlin<'a>(selected: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let meta_path = env
            .call_method(
                selected,
                "getMetaPath",
                format!("()L{};", KotlinClassName::ResourcePath.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let meta_path = ResourcePath::try_from_kotlin(meta_path.as_obj(), env)?;
        let stream_path = env
            .call_method(
                selected,
                "getStreamPath",
                format!("()L{};", KotlinClassName::ResourcePath.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let stream_path = Option::<ResourcePath>::try_from_kotlin(stream_path.as_obj(), env)?;
        Ok(Selected {
            meta_path,
            stream_path,
        })
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

impl ToProtobuf<types::Video, Option<String>> for Video {
    fn to_protobuf(&self, addon_name: &Option<String>) -> types::Video {
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
        }
    }
}

impl ToProtobuf<types::MetaItem, Option<String>> for MetaItem {
    fn to_protobuf(&self, addon_name: &Option<String>) -> types::MetaItem {
        types::MetaItem {
            id: self.id.to_string(),
            r#type: self.r#type.to_string(),
            name: self.name.to_string(),
            poster_shape: self.poster_shape.to_protobuf(&()) as i32,
            poster: self.poster.clone(),
            background: self.background.clone(),
            logo: self.logo.clone(),
            description: self.description.clone(),
            release_info: self.release_info.clone(),
            runtime: self.runtime.clone(),
            released: self.released.to_protobuf(&()),
            links: self.links.to_protobuf(&()),
            trailer_streams: self.trailer_streams.to_protobuf(&(None, None, None)),
            videos: self.videos.to_protobuf(&(addon_name)),
            behavior_hints: self.behavior_hints.to_protobuf(&()),
            deep_links: MetaItemDeepLinks::from(self).to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::MetaDetails, Ctx> for MetaDetails {
    fn to_protobuf(&self, ctx: &Ctx) -> models::MetaDetails {
        let meta_item = self
            .meta_items
            .iter()
            .find(|meta_item| meta_item.content.is_ready())
            .or_else(|| {
                if self
                    .meta_items
                    .iter()
                    .all(|meta_item| meta_item.content.is_err())
                {
                    self.meta_items.first()
                } else {
                    self.meta_items
                        .iter()
                        .find(|catalog| catalog.content.is_loading())
                }
            });
        let meta_request = meta_item.map(|item| &item.request);
        let title = meta_item
            .and_then(|meta_item| meta_item.content.as_ref().ready())
            .map(|meta_item| {
                meta_item
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
                            &meta_item.name,
                            &video.title,
                            &series_info.season,
                            &series_info.episode
                        ),
                        _ => format!("{} - {}", &meta_item.name, &video.title),
                    })
                    .unwrap_or_else(|| meta_item.name.to_owned())
            });
        models::MetaDetails {
            selected: self.selected.to_protobuf(&()),
            title,
            meta_item: meta_item.to_protobuf(ctx),
            streams: self.streams.to_protobuf(&(ctx, meta_request)),
        }
    }
}
