use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::{models, types};
use boolinator::Boolinator;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::meta_details::{MetaDetails, Selected};
use stremio_core::types::addon::ResourcePath;
use stremio_core::types::resource::{MetaItem, SeriesInfo, Video};
use stremio_deeplinks::MetaItemDeepLinks;

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

impl<'a> TryIntoKotlin<'a, ()> for Selected {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let meta_path = self.meta_path.try_into_kotlin(&(), env)?.auto_local(env);
        let stream_path = self.stream_path.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::MetaDetails_Selected).unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::ResourcePath.value(),
                KotlinClassName::ResourcePath.value()
            ),
            &[meta_path.as_obj().into(), stream_path.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SeriesInfo {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let season = (self.season as i64).into();
        let episode = (self.episode as i64).into();
        env.new_object(
            classes.get(&KotlinClassName::Video_SeriesInfo).unwrap(),
            "(JJ)V",
            &[season, episode],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Video {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let id = self.id.try_into_kotlin(&(), env)?.auto_local(env);
        let title = self.title.try_into_kotlin(&(), env)?.auto_local(env);
        let released = self.released.try_into_kotlin(&(), env)?.auto_local(env);
        let overview = self.overview.try_into_kotlin(&(), env)?.auto_local(env);
        let thumbnail = self.thumbnail.try_into_kotlin(&(), env)?.auto_local(env);
        let streams = self.streams.try_into_kotlin(&(), env)?.auto_local(env);
        let series_info = self.series_info.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Video).unwrap(),
            format!(
                "(L{};L{};L{};L{};L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::Date.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                "java/util/List",
                KotlinClassName::Video_SeriesInfo.value(),
            ),
            &[
                id.as_obj().into(),
                title.as_obj().into(),
                released.as_obj().into(),
                overview.as_obj().into(),
                thumbnail.as_obj().into(),
                streams.as_obj().into(),
                series_info.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for MetaItem {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let id = self.id.try_into_kotlin(&(), env)?.auto_local(env);
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let poster = self.poster.try_into_kotlin(&(), env)?.auto_local(env);
        let background = self.background.try_into_kotlin(&(), env)?.auto_local(env);
        let logo = self.logo.try_into_kotlin(&(), env)?.auto_local(env);
        let description = self.description.try_into_kotlin(&(), env)?.auto_local(env);
        let release_info = self.release_info.try_into_kotlin(&(), env)?.auto_local(env);
        let runtime = self.runtime.try_into_kotlin(&(), env)?.auto_local(env);
        let released = self.released.try_into_kotlin(&(), env)?.auto_local(env);
        let poster_shape = self.poster_shape.try_into_kotlin(&(), env)?.auto_local(env);
        let links = self.links.try_into_kotlin(&(), env)?.auto_local(env);
        let trailer_streams = self
            .trailer_streams
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let videos = self.videos.try_into_kotlin(&(), env)?.auto_local(env);
        let behavior_hints = self
            .behavior_hints
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let deep_links = MetaItemDeepLinks::from(self)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::MetaItem).unwrap(),
            format!(
                "(L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::Date.value(),
                KotlinClassName::PosterShape.value(),
                "java/util/List",
                "java/util/List",
                "java/util/List",
                KotlinClassName::MetaItemBehaviorHints.value(),
                KotlinClassName::MetaItemDeepLinks.value()
            ),
            &[
                id.as_obj().into(),
                r#type.as_obj().into(),
                name.as_obj().into(),
                poster.as_obj().into(),
                background.as_obj().into(),
                logo.as_obj().into(),
                description.as_obj().into(),
                release_info.as_obj().into(),
                runtime.as_obj().into(),
                released.as_obj().into(),
                poster_shape.as_obj().into(),
                links.as_obj().into(),
                trailer_streams.as_obj().into(),
                videos.as_obj().into(),
                behavior_hints.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, Ctx> for MetaDetails {
    fn try_into_kotlin(&self, ctx: &Ctx, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let selected = self.selected.try_into_kotlin(&(), env)?.auto_local(env);
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
            })
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let streams = self
            .streams
            .iter()
            .filter_map(|streams_resource| {
                ctx.profile
                    .addons
                    .iter()
                    .find(|addon| addon.transport_url == streams_resource.request.base)
                    .map(|addon| (streams_resource, addon))
            })
            .map(|(streams_resource, addon)| {
                let addon_name = addon.manifest.name.to_owned();
                streams_resource.try_into_kotlin(&(Some(addon_name), ()), env)
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let meta_addon_name = meta_item
            .and_then(|meta_item| {
                ctx.profile
                    .addons
                    .iter()
                    .find(|addon| addon.transport_url == meta_item.request.base)
            })
            .map(|addon| addon.manifest.name.to_owned());
        let meta_item = meta_item
            .try_into_kotlin(&(meta_addon_name, ()), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::MetaDetails).unwrap(),
            format!(
                "(L{};Ljava/lang/String;L{};Ljava/util/List;)V",
                KotlinClassName::MetaDetails_Selected.value(),
                KotlinClassName::ResourceLoadable.value(),
            ),
            &[
                selected.as_obj().into(),
                title.as_obj().into(),
                meta_item.as_obj().into(),
                streams.as_obj().into(),
            ],
        )
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

impl ToProtobuf<types::Video, ()> for Video {
    fn to_protobuf(&self, _args: &()) -> types::Video {
        types::Video {
            id: self.id.to_string(),
            title: self.title.to_string(),
            released: self.released.to_protobuf(&()),
            overview: self.overview.clone(),
            thumbnail: self.thumbnail.clone(),
            streams: self.streams.to_protobuf(&()),
            series_info: self.series_info.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<types::MetaItem, ()> for MetaItem {
    fn to_protobuf(&self, _args: &()) -> types::MetaItem {
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
            trailer_streams: self.trailer_streams.to_protobuf(&()),
            videos: self.videos.to_protobuf(&()),
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
            streams: self.streams.to_protobuf(ctx),
        }
    }
}
