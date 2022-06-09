use stremio_core::deep_links::StreamDeepLinks;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::{Stream, StreamSource};

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::types;

impl ToProtobuf<types::stream::Source, ()> for StreamSource {
    fn to_protobuf(&self, _args: &()) -> types::stream::Source {
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

impl
    ToProtobuf<
        types::Stream,
        (
            Option<String>,
            Option<&ResourceRequest>,
            Option<&ResourceRequest>,
        ),
    > for Stream
{
    fn to_protobuf(
        &self,
        (addon_name, stream_request, meta_request): &(
            Option<String>,
            Option<&ResourceRequest>,
            Option<&ResourceRequest>,
        ),
    ) -> types::Stream {
        let deep_links = if stream_request.is_some() && meta_request.is_some() {
            StreamDeepLinks::from((self, stream_request.unwrap(), meta_request.unwrap()))
        } else {
            StreamDeepLinks::from(self)
        };
        types::Stream {
            name: self.name.to_owned().or(addon_name.to_owned()),
            description: self.description.clone(),
            thumbnail: self.thumbnail.clone(),
            behavior_hints: types::StreamBehaviorHints {
                not_web_ready: self.behavior_hints.not_web_ready,
                binge_group: self.behavior_hints.binge_group.to_owned(),
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
