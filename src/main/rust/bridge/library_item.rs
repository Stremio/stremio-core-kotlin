use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::library::{LibraryItem, LibraryItemBehaviorHints, LibraryItemState};
use stremio_deeplinks::LibraryItemDeepLinks;

impl<'a> TryIntoKotlin<'a, ()> for LibraryItemBehaviorHints {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let default_video_id = self
            .default_video_id
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryItemBehaviorHints)
                .unwrap(),
            format!("(L{};)V", KotlinClassName::String.value()),
            &[default_video_id.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for LibraryItemState {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        env.new_object(
            classes.get(&KotlinClassName::LibraryItemState).unwrap(),
            "(JJ)V",
            &[
                (self.time_offset as i64).into(),
                (self.duration as i64).into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for LibraryItemDeepLinks {
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
        let player = self.player.try_into_kotlin(&(), env)?.auto_local(env);
        let external_player = self
            .external_player
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LibraryItemDeepLinks).unwrap(),
            format!(
                "(L{};L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::ExternalPlayerLink.value()
            ),
            &[
                meta_details_videos.as_obj().into(),
                meta_details_streams.as_obj().into(),
                player.as_obj().into(),
                external_player.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for LibraryItem {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let id = self.id.try_into_kotlin(&(), env)?.auto_local(env);
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let poster = self.poster.try_into_kotlin(&(), env)?.auto_local(env);
        let poster_shape = self.poster_shape.try_into_kotlin(&(), env)?.auto_local(env);
        let state = self.state.try_into_kotlin(&(), env)?.auto_local(env);
        let behavior_hints = self
            .behavior_hints
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let deep_links = LibraryItemDeepLinks::from(self)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LibraryItem).unwrap(),
            format!(
                "(L{};L{};L{};L{};L{};L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::PosterShape.value(),
                KotlinClassName::LibraryItemState.value(),
                KotlinClassName::LibraryItemBehaviorHints.value(),
                KotlinClassName::LibraryItemDeepLinks.value()
            ),
            &[
                id.as_obj().into(),
                r#type.as_obj().into(),
                name.as_obj().into(),
                poster.as_obj().into(),
                poster_shape.as_obj().into(),
                state.as_obj().into(),
                behavior_hints.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}
