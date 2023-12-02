use stremio_core::deep_links::MetaItemDeepLinks;
use stremio_core::models::common::{Loadable, ResourceError};
use stremio_core::models::ctx::Ctx;
use stremio_core::models::link::LinkError;
use stremio_core::models::streaming_server::{PlaybackDevice, Settings};
use stremio_core::runtime::EnvError;
use stremio_core::types::addon::{Descriptor, DescriptorPreview, ResourcePath, ResourceRequest};
use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use stremio_core::types::library::LibraryItem;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles};
use stremio_core::types::streaming_server::Statistics;
use stremio_watched_bitfield::WatchedBitField;
use url::Url;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;
use crate::protobuf::stremio::core::models::PlaybackDevices;

impl ToProtobuf<models::loadable_page::Content, (&Ctx, &ResourceRequest)>
    for Loadable<Vec<MetaItemPreview>, ResourceError>
{
    fn to_protobuf(
        &self,
        (ctx, request): &(&Ctx, &ResourceRequest),
    ) -> models::loadable_page::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_page::Content::Ready(models::Page {
                meta_items: ready.to_protobuf(&(*ctx, *request)),
            }),
            Loadable::Err(error) => models::loadable_page::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_page::Content::Loading(models::Loading {}),
        }
    }
}

impl
    ToProtobuf<
        models::loadable_meta_item::Content,
        (
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    > for Loadable<MetaItem, ResourceError>
{
    fn to_protobuf(
        &self,
        (library_item, watched, addon_name, meta_request): &(
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    ) -> models::loadable_meta_item::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_meta_item::Content::Ready(
                ready.to_protobuf(&(*library_item, *watched, *addon_name, *meta_request)),
            ),
            Loadable::Err(error) => models::loadable_meta_item::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_meta_item::Content::Loading(models::Loading {}),
        }
    }
}

impl
    ToProtobuf<
        models::loadable_streams::Content,
        (&Ctx, &String, &ResourceRequest, Option<&ResourceRequest>),
    > for Loadable<Vec<Stream>, ResourceError>
{
    fn to_protobuf(
        &self,
        (ctx, addon_name, stream_request, meta_request): &(
            &Ctx,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    ) -> models::loadable_streams::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_streams::Content::Ready(models::Streams {
                streams: ready.to_protobuf(&(
                    Some(*ctx),
                    Some(*addon_name),
                    Some(*stream_request),
                    *meta_request,
                )),
            }),
            Loadable::Err(error) => models::loadable_streams::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_streams::Content::Loading(models::Loading {}),
        }
    }
}

impl
    ToProtobuf<
        models::loadable_stream::Content,
        (&Ctx, &String, &ResourceRequest, Option<&ResourceRequest>),
    > for Loadable<Option<Stream>, ResourceError>
{
    fn to_protobuf(
        &self,
        (ctx, addon_name, stream_request, meta_request): &(
            &Ctx,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    ) -> models::loadable_stream::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_stream::Content::Ready(models::OptionStream {
                    stream: ready.to_protobuf(&(
                        Some(*ctx),
                        Some(*addon_name),
                        Some(*stream_request),
                        *meta_request,
                    )),
                })
            }
            Loadable::Err(error) => models::loadable_stream::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_stream::Content::Loading(models::Loading {}),
        }
    }
}

impl ToProtobuf<models::loadable_subtitles::Content, Option<&String>>
    for Loadable<Vec<Subtitles>, ResourceError>
{
    fn to_protobuf(&self, addon_name: &Option<&String>) -> models::loadable_subtitles::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_subtitles::Content::Ready(models::Subtitles {
                    subtitles: ready.to_protobuf(addon_name),
                })
            }
            Loadable::Err(error) => models::loadable_subtitles::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_subtitles::Content::Loading(models::Loading {}),
        }
    }
}

impl ToProtobuf<models::LoadableSettings, ()> for Loadable<Settings, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableSettings {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_settings::Content::Ready(ready.to_protobuf(&()))
            }
            Loadable::Err(error) => models::loadable_settings::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_settings::Content::Loading(models::Loading {}),
        };
        models::LoadableSettings {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableBaseUrl, ()> for Loadable<Url, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableBaseUrl {
        let content = match &self {
            Loadable::Ready(ready) => models::loadable_base_url::Content::Ready(ready.to_string()),
            Loadable::Err(error) => models::loadable_base_url::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_base_url::Content::Loading(models::Loading {}),
        };
        models::LoadableBaseUrl {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableCode, ()> for Loadable<LinkCodeResponse, LinkError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableCode {
        let content = match &self {
            Loadable::Ready(ready) => models::loadable_code::Content::Ready(ready.to_protobuf(&())),
            Loadable::Err(error) => models::loadable_code::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_code::Content::Loading(models::Loading {}),
        };
        models::LoadableCode {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableAuthKey, ()> for Loadable<LinkAuthKey, LinkError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableAuthKey {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_auth_key::Content::Ready(ready.to_protobuf(&()))
            }
            Loadable::Err(error) => models::loadable_auth_key::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_auth_key::Content::Loading(models::Loading {}),
        };
        models::LoadableAuthKey {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableTorrent, ()> for Loadable<ResourcePath, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableTorrent {
        let content = match &self {
            Loadable::Ready(ready) => {
                let deeplinks = MetaItemDeepLinks::from(ready).to_protobuf(&());
                models::loadable_torrent::Deeplinks::Ready(deeplinks)
            }
            Loadable::Err(error) => models::loadable_torrent::Deeplinks::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_torrent::Deeplinks::Loading(models::Loading {}),
        };
        models::LoadableTorrent {
            deeplinks: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadablePlaybackDevices, ()> for Loadable<Vec<PlaybackDevice>, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadablePlaybackDevices {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_playback_devices::Content::Ready(PlaybackDevices {
                    devices: ready.to_protobuf(&()),
                })
            }
            Loadable::Err(error) => {
                models::loadable_playback_devices::Content::Error(models::Error {
                    message: error.to_string(),
                })
            }
            Loadable::Loading => {
                models::loadable_playback_devices::Content::Loading(models::Loading {})
            }
        };
        models::LoadablePlaybackDevices {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableStatistics, ()> for Loadable<Statistics, EnvError> {
    fn to_protobuf(&self, _args: &()) -> models::LoadableStatistics {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_statistics::Content::Ready(ready.to_protobuf(&()))
            }
            Loadable::Err(error) => models::loadable_statistics::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_statistics::Content::Loading(models::Loading {}),
        };
        models::LoadableStatistics {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::loadable_addon_catalog::Content, Ctx>
    for Loadable<Vec<DescriptorPreview>, ResourceError>
{
    fn to_protobuf(&self, ctx: &Ctx) -> models::loadable_addon_catalog::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_addon_catalog::Content::Ready(models::Addons {
                    items: ready.to_protobuf(ctx),
                })
            }
            Loadable::Err(error) => models::loadable_addon_catalog::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => {
                models::loadable_addon_catalog::Content::Loading(models::Loading {})
            }
        }
    }
}

impl ToProtobuf<models::loadable_descriptor::Content, Ctx> for Loadable<Descriptor, EnvError> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::loadable_descriptor::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_descriptor::Content::Ready(ready.to_protobuf(ctx))
            }
            Loadable::Err(error) => models::loadable_descriptor::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_descriptor::Content::Loading(models::Loading {}),
        }
    }
}
