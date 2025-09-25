use hex::FromHex;
use stremio_core::deep_links::StreamDeepLinks;
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::{
    Stream, StreamBehaviorHints, StreamProxyHeaders, StreamSource,
};
use url::Url;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<StreamSource> for types::stream::Source {
    fn from_protobuf(&self) -> StreamSource {
        match self {
            types::stream::Source::Url(source) => StreamSource::Url {
                url: source.url.from_protobuf(),
            },
            types::stream::Source::YouTube(source) => StreamSource::YouTube {
                yt_id: source.yt_id.to_owned(),
            },
            types::stream::Source::Tramvai(source) => StreamSource::Torrent {
                info_hash: <[u8; 20]>::from_hex(source.info_hash.as_str())
                    .expect("Stream.info_hash parse failed"),
                file_idx: source.file_idx.map(|idx| idx as u16),
                announce: source.announce.clone(),
                file_must_include: source.file_must_include.to_owned(),
            },
            types::stream::Source::External(source) => StreamSource::External {
                external_url: source.external_url.from_protobuf(),
                android_tv_url: source.android_tv_url.from_protobuf(),
                tizen_url: None,
                webos_url: None,
            },
            types::stream::Source::PlayerFrame(source) => StreamSource::PlayerFrame {
                player_frame_url: source.player_frame_url.from_protobuf(),
            },
            types::stream::Source::Rar(source) => StreamSource::Rar {
                rar_urls: source.rar_urls.from_protobuf(),
                file_idx: source.file_idx.map(|idx| idx as u16),
                file_must_include: source.file_must_include.to_owned(),
            },
            types::stream::Source::Zip(source) => StreamSource::Zip {
                zip_urls: source.zip_urls.from_protobuf(),
                file_idx: source.file_idx.map(|idx| idx as u16),
                file_must_include: source.file_must_include.to_owned(),
            },
        }
    }
}

impl FromProtobuf<StreamProxyHeaders> for types::StreamProxyHeaders {
    fn from_protobuf(&self) -> StreamProxyHeaders {
        StreamProxyHeaders {
            request: self.request.to_owned(),
            response: self.response.to_owned(),
        }
    }
}

impl FromProtobuf<Stream> for types::Stream {
    fn from_protobuf(&self) -> Stream {
        Stream {
            source: self.source.from_protobuf().unwrap(),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            thumbnail: self.thumbnail.to_owned(),
            subtitles: self.subtitles.from_protobuf(),
            behavior_hints: StreamBehaviorHints {
                not_web_ready: self.behavior_hints.not_web_ready,
                binge_group: self.behavior_hints.binge_group.to_owned(),
                country_whitelist: Some(self.behavior_hints.country_whitelist.to_owned()),
                proxy_headers: self.behavior_hints.proxy_headers.from_protobuf(),
                filename: self.behavior_hints.filename.to_owned(),
                video_hash: self.behavior_hints.video_hash.to_owned(),
                video_size: self.behavior_hints.video_size,
                other: Default::default(),
            },
        }
    }
}

impl ToProtobuf<types::stream::Source, ()> for StreamSource {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::stream::Source {
        match self {
            StreamSource::Url { url } => types::stream::Source::Url(types::stream::Url {
                url: url.to_string(),
            }),
            StreamSource::YouTube { yt_id } => {
                types::stream::Source::YouTube(types::stream::YouTube {
                    yt_id: yt_id.to_string(),
                })
            }
            StreamSource::Torrent {
                info_hash,
                file_idx,
                announce,
                file_must_include,
            } => types::stream::Source::Tramvai(types::stream::Tramvai {
                info_hash: hex::encode(info_hash),
                file_idx: file_idx.map(|idx| idx as i32),
                announce: announce.clone(),
                file_must_include: file_must_include.to_owned(),
            }),
            StreamSource::External {
                external_url,
                android_tv_url,
                ..
            } => types::stream::Source::External(types::stream::External {
                external_url: external_url.to_protobuf::<E>(&()),
                android_tv_url: android_tv_url.to_protobuf::<E>(&()),
            }),
            StreamSource::PlayerFrame { player_frame_url } => {
                types::stream::Source::PlayerFrame(types::stream::PlayerFrame {
                    player_frame_url: player_frame_url.to_string(),
                })
            }
            StreamSource::Rar {
                rar_urls,
                file_idx,
                file_must_include,
            } => types::stream::Source::Rar(types::stream::Rar {
                rar_urls: rar_urls.to_protobuf::<E>(&()),
                file_idx: file_idx.map(|idx| idx as i32),
                file_must_include: file_must_include.to_owned(),
            }),
            StreamSource::Zip {
                zip_urls,
                file_idx,
                file_must_include,
            } => types::stream::Source::Zip(types::stream::Zip {
                zip_urls: zip_urls.to_protobuf::<E>(&()),
                file_idx: file_idx.map(|idx| idx as i32),
                file_must_include: file_must_include.to_owned(),
            }),
        }
    }
}

impl ToProtobuf<types::StreamProxyHeaders, ()> for StreamProxyHeaders {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::StreamProxyHeaders {
        types::StreamProxyHeaders {
            request: self.request.to_owned(),
            response: self.response.to_owned(),
        }
    }
}

impl
    ToProtobuf<
        types::Stream,
        (
            Option<&Ctx>,
            Option<&Url>,
            Option<&String>,
            Option<&ResourceRequest>,
            Option<&ResourceRequest>,
        ),
    > for Stream
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, addon_name, stream_request, meta_request): &(
            Option<&Ctx>,
            Option<&Url>,
            Option<&String>,
            Option<&ResourceRequest>,
            Option<&ResourceRequest>,
        ),
    ) -> types::Stream {
        // in calls that have None for ctx this would panic if we don't set it to default.
        let settings = ctx
            .map(|ctx| ctx.profile.settings.to_owned())
            .unwrap_or_default();

        let deep_links = match (stream_request, meta_request) {
            (Some(stream_request), Some(meta_request)) => StreamDeepLinks::from((
                self,
                *stream_request,
                *meta_request,
                &streaming_server_url.map(Clone::clone),
                &settings,
            )),
            _ => StreamDeepLinks::from((self, &streaming_server_url.map(Clone::clone), &settings)),
        };

        types::Stream {
            name: self.name.to_owned().or_else(|| addon_name.cloned()),
            description: self.description.clone(),
            thumbnail: self.thumbnail.clone(),
            subtitles: self.subtitles.to_protobuf::<E>(addon_name),
            behavior_hints: types::StreamBehaviorHints {
                not_web_ready: self.behavior_hints.not_web_ready,
                binge_group: self.behavior_hints.binge_group.to_owned(),
                country_whitelist: self
                    .behavior_hints
                    .country_whitelist
                    .to_owned()
                    .unwrap_or_default(),
                proxy_headers: self.behavior_hints.proxy_headers.to_protobuf::<E>(&()),
                filename: self.behavior_hints.filename.to_owned(),
                video_hash: self.behavior_hints.video_hash.to_owned(),
                video_size: self.behavior_hints.video_size,
            },
            deep_links: types::StreamDeepLinks {
                player: deep_links.player,
                external_player: types::stream_deep_links::ExternalPlayerLink {
                    download: deep_links.external_player.download,
                    streaming: deep_links.external_player.streaming,
                    open_player: deep_links
                        .external_player
                        .open_player
                        .as_ref()
                        .map(|core_op| types::stream_deep_links::OpenPlayerLink {
                            ios: core_op.ios.clone(),
                            macos: core_op.macos.clone(),
                            visionos: core_op.visionos.clone(),
                        }),
                },
            },
            source: Some(self.source.to_protobuf::<E>(&())),
        }
    }
}
