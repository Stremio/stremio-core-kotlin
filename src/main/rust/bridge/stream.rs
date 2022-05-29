use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::types::resource::{Stream, StreamBehaviorHints, StreamSource};
use stremio_deeplinks::StreamDeepLinks;

use crate::bridge::{ToProtobuf, ToProtobufAny, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl<'a> TryIntoKotlin<'a, ()> for StreamDeepLinks {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let player = self.player.try_into_kotlin(&(), env)?.auto_local(env);
        let external_player = self
            .external_player
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::StreamDeepLinks).unwrap(),
            format!(
                "(Ljava/lang/String;L{};)V",
                KotlinClassName::ExternalPlayerLink.value()
            ),
            &[player.as_obj().into(), external_player.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for StreamBehaviorHints {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let not_web_ready = self.not_web_ready.into();
        let binge_group = self.binge_group.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::StreamBehaviorHints).unwrap(),
            "(ZLjava/lang/String;)V",
            &[not_web_ready, binge_group.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for StreamSource {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            StreamSource::Url { url } => {
                let url = url.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::StreamSource_Url).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[url.as_obj().into()],
                )
            }
            StreamSource::YouTube { yt_id } => {
                let yt_id = yt_id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::StreamSource_YouTube).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[yt_id.as_obj().into()],
                )
            }
            StreamSource::Torrent {
                info_hash,
                file_idx,
                announce,
            } => {
                let info_hash = hex::encode(info_hash)
                    .try_into_kotlin(&(), env)?
                    .auto_local(env);
                let file_idx = match file_idx {
                    Some(file_idx) => env.new_object(
                        classes.get(&KotlinClassName::Integer).unwrap(),
                        "(I)V",
                        &[(*file_idx as i32).into()],
                    )?,
                    _ => JObject::null(),
                };
                let file_idx = file_idx.auto_local(env);
                let announce = announce.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::StreamSource_Torrent).unwrap(),
                    "(Ljava/lang/String;Ljava/lang/Integer;Ljava/util/List;)V",
                    &[
                        info_hash.as_obj().into(),
                        file_idx.as_obj().into(),
                        announce.as_obj().into(),
                    ],
                )
            }
            StreamSource::External { external_url } => {
                let external_url = external_url.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::StreamSource_External)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[external_url.as_obj().into()],
                )
            }
            StreamSource::PlayerFrame { player_frame_url } => {
                let player_frame_url = player_frame_url.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::StreamSource_PlayerFrame)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[player_frame_url.as_obj().into()],
                )
            }
        }
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Stream {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let source = self.source.try_into_kotlin(&(), env)?.auto_local(env);
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let description = self.description.try_into_kotlin(&(), env)?.auto_local(env);
        let thumbnail = self.thumbnail.try_into_kotlin(&(), env)?.auto_local(env);
        let behavior_hints = self
            .behavior_hints
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        let deep_links = StreamDeepLinks::from(self)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Stream).unwrap(),
            format!(
                "(L{};Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;L{};L{};)V",
                KotlinClassName::StreamSource.value(),
                KotlinClassName::StreamBehaviorHints.value(),
                KotlinClassName::StreamDeepLinks.value()
            ),
            &[
                source.as_obj().into(),
                name.as_obj().into(),
                description.as_obj().into(),
                thumbnail.as_obj().into(),
                behavior_hints.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}

impl ToProtobufAny<types::stream::Source, ()> for StreamSource {
    fn to_protobuf(&self, _args: &()) -> types::stream::Source {
        match self {
            StreamSource::Url { url } => types::stream::Source::Url(types::stream::Url {
                url: url.to_string(),
            }),
            StreamSource::YouTube { yt_id } => {
                types::stream::Source::Youtube(types::stream::YouTube {
                    yt_id: yt_id.to_string(),
                })
            }
            StreamSource::Torrent {
                info_hash,
                file_idx,
                announce,
            } => types::stream::Source::Torrent(types::stream::Torrent {
                info_hash: hex::encode(info_hash),
                file_idx: file_idx.map(|idx| idx as i32),
                announce: announce.clone(),
            }),
            StreamSource::External { external_url } => {
                types::stream::Source::External(types::stream::External {
                    external_url: external_url.to_string(),
                })
            }
            StreamSource::PlayerFrame { player_frame_url } => {
                types::stream::Source::PlayerFrame(types::stream::PlayerFrame {
                    player_frame_url: player_frame_url.to_string(),
                })
            }
        }
    }
}

impl ToProtobuf<types::Stream, ()> for Stream {
    fn to_protobuf(&self, _args: &()) -> types::Stream {
        let deep_links = StreamDeepLinks::from(self);
        types::Stream {
            name: self.name.clone(),
            description: self.description.clone(),
            thumbnail: self.thumbnail.clone(),
            behavior_hints: types::StreamBehaviorHints {
                not_web_ready: self.behavior_hints.not_web_ready,
                binge_group: self.behavior_hints.binge_group.clone(),
                country_whitelist: vec![],
                headers: Default::default(),
            },
            deep_links: types::StreamDeepLinks {
                player: deep_links.player,
                external_player: types::stream_deep_links::ExternalPlayerLink {
                    href: deep_links.external_player.href,
                    download: deep_links.external_player.download,
                },
            },
            source: Some(self.source.to_protobuf(&())),
        }
    }
}
