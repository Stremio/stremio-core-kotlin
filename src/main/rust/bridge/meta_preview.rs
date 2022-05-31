use chrono::{DateTime, Utc};
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::resource::{MetaItemBehaviorHints, MetaItemPreview, PosterShape};
use stremio_deeplinks::MetaItemDeepLinks;

use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for MetaItemBehaviorHints {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let default_video_id = env
            .call_method(
                value,
                "getDefaultVideoId",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let default_video_id = Option::<String>::try_from_kotlin(default_video_id.as_obj(), env)?;
        let featured_video_id = env
            .call_method(
                value,
                "getFeaturedVideoId",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let featured_video_id = Option::<String>::try_from_kotlin(featured_video_id.as_obj(), env)?;
        let has_scheduled_videos = env
            .call_method(value, "getHasScheduledVideos", "()Z", &[])?
            .z()?;
        Ok(MetaItemBehaviorHints {
            default_video_id,
            featured_video_id,
            has_scheduled_videos,
        })
    }
}

impl TryFromKotlin for MetaItemDeepLinks {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let meta_details_videos = env
            .call_method(
                value,
                "getMetaDetailsVideos",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let meta_details_videos =
            Option::<String>::try_from_kotlin(meta_details_videos.as_obj(), env)?;
        let meta_details_streams = env
            .call_method(
                value,
                "getMetaDetailsStreams",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let meta_details_streams =
            Option::<String>::try_from_kotlin(meta_details_streams.as_obj(), env)?;
        Ok(MetaItemDeepLinks {
            meta_details_videos,
            meta_details_streams,
        })
    }
}

impl TryFromKotlin for MetaItemPreview {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let id = env
            .call_method(
                value,
                "getId",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let id = String::try_from_kotlin(id.as_obj(), env)?;
        let r#type = env
            .call_method(
                value,
                "getType",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let r#type = String::try_from_kotlin(r#type.as_obj(), env)?;
        let name = env
            .call_method(
                value,
                "getName",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let name = String::try_from_kotlin(name.as_obj(), env)?;
        let poster = env
            .call_method(
                value,
                "getPoster",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let poster = Option::<String>::try_from_kotlin(poster.as_obj(), env)?;
        let background = env
            .call_method(
                value,
                "getBackground",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let background = Option::<String>::try_from_kotlin(background.as_obj(), env)?;
        let logo = env
            .call_method(
                value,
                "getLogo",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let logo = Option::<String>::try_from_kotlin(logo.as_obj(), env)?;
        let description = env
            .call_method(
                value,
                "getDescription",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let description = Option::<String>::try_from_kotlin(description.as_obj(), env)?;
        let release_info = env
            .call_method(
                value,
                "getReleaseInfo",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let release_info = Option::<String>::try_from_kotlin(release_info.as_obj(), env)?;
        let runtime = env
            .call_method(
                value,
                "getRuntime",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let runtime = Option::<String>::try_from_kotlin(runtime.as_obj(), env)?;
        let released = env
            .call_method(
                value,
                "getReleased",
                format!("()L{};", KotlinClassName::Date.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let released = Option::<DateTime<Utc>>::try_from_kotlin(released.as_obj(), env)?;
        let poster_shape = env
            .call_method(
                value,
                "getPosterShape",
                format!("()L{};", KotlinClassName::PosterShape.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let poster_shape = PosterShape::try_from_kotlin(poster_shape.as_obj(), env)?;
        let behavior_hints = env
            .call_method(
                value,
                "getBehaviorHints",
                format!("()L{};", KotlinClassName::MetaItemBehaviorHints.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let behavior_hints = MetaItemBehaviorHints::try_from_kotlin(behavior_hints.as_obj(), env)?;
        Ok(MetaItemPreview {
            id,
            r#type,
            name,
            poster,
            background,
            logo,
            description,
            release_info,
            runtime,
            released,
            poster_shape,
            links: Default::default(),
            behavior_hints,
            trailer_streams: Default::default(),
        })
    }
}

impl ToProtobuf<types::MetaItemBehaviorHints, ()> for MetaItemBehaviorHints {
    fn to_protobuf(&self, _args: &()) -> types::MetaItemBehaviorHints {
        types::MetaItemBehaviorHints {
            default_video_id: self.default_video_id.clone(),
            featured_video_id: self.featured_video_id.clone(),
            has_scheduled_videos: self.has_scheduled_videos,
        }
    }
}

impl ToProtobuf<types::MetaItemDeepLinks, ()> for MetaItemDeepLinks {
    fn to_protobuf(&self, _args: &()) -> types::MetaItemDeepLinks {
        types::MetaItemDeepLinks {
            meta_details_videos: self.meta_details_videos.clone(),
            meta_details_streams: self.meta_details_streams.clone(),
            player: None, // TODO populate
        }
    }
}

impl ToProtobuf<types::MetaItemPreview, ()> for MetaItemPreview {
    fn to_protobuf(&self, _args: &()) -> types::MetaItemPreview {
        types::MetaItemPreview {
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
            behavior_hints: self.behavior_hints.to_protobuf(&()),
            deep_links: MetaItemDeepLinks::from(self).to_protobuf(&()),
        }
    }
}
