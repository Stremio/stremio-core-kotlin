use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use chrono::{DateTime, Utc};
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::resource::{Link, MetaItemBehaviorHints, MetaItemPreview, PosterShape};
use stremio_deeplinks::MetaItemDeepLinks;

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

impl<'a> TryIntoKotlin<'a, ()> for MetaItemBehaviorHints {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let default_video_id = self
            .default_video_id
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let featured_video_id = self
            .featured_video_id
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let has_scheduled_videos = self.has_scheduled_videos.into();
        env.new_object(
            classes
                .get(&KotlinClassName::MetaItemBehaviorHints)
                .unwrap(),
            format!(
                "(L{};L{};Z)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[
                default_video_id.as_obj().into(),
                featured_video_id.as_obj().into(),
                has_scheduled_videos,
            ],
        )
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

impl<'a> TryIntoKotlin<'a, ()> for MetaItemDeepLinks {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let meta_details_videos = self
            .meta_details_videos
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let meta_details_streams = self
            .meta_details_streams
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::MetaItemDeepLinks).unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[
                meta_details_videos.as_obj().into(),
                meta_details_streams.as_obj().into(),
            ],
        )
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
        let links = env
            .call_method(value, "getLinks", "()Ljava/util/List;", &[])?
            .l()?
            .auto_local(env);
        let links = Vec::<Link>::try_from_kotlin(links.as_obj(), env)?;
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
            links,
            behavior_hints,
            trailer_streams: Default::default(),
        })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for MetaItemPreview {
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
        let behavior_hints = self
            .behavior_hints
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let deep_links = MetaItemDeepLinks::from(self)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::MetaItemPreview).unwrap(),
            format!(
                "(L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};L{};)V",
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
                behavior_hints.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}
