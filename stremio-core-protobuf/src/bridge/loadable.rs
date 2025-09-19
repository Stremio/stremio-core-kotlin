use itertools::Itertools;
use url::Url;

use stremio_core::{
    deep_links::MetaItemDeepLinks,
    models::{
        common::{Loadable, ResourceError},
        ctx::{Ctx, CtxError},
        link::LinkError,
        streaming_server::PlaybackDevice,
    },
    runtime::EnvError,
    types::{
        addon::{Descriptor, ResourcePath, ResourceRequest},
        api::{GetModalResponse, GetNotificationResponse, LinkAuthKey, LinkCodeResponse},
        library::LibraryItem,
        resource::{MetaItem, MetaItemPreview, Stream, Subtitles},
        streaming_server::{Settings, Statistics},
        watched_bitfield::WatchedBitField,
    },
};

use crate::{
    bridge::ToProtobuf,
    protobuf::stremio::core::models::{self, LoadedModal, LoadedNotification, PlaybackDevices},
};

impl ToProtobuf<models::loadable_page::Content, (&Ctx, &ResourceRequest)>
    for Loadable<Vec<MetaItemPreview>, ResourceError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, request): &(&Ctx, &ResourceRequest),
    ) -> models::loadable_page::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_page::Content::Ready(models::Page {
                meta_items: ready
                    .iter()
                    .unique_by(|meta_item| &meta_item.id)
                    .map(|meta_item| meta_item.to_owned())
                    .collect_vec()
                    .to_protobuf::<E>(&(*ctx, *request)),
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
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    > for Loadable<MetaItem, ResourceError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, library_item, watched, addon_name, meta_request): &(
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
            Option<&String>,
            &ResourceRequest,
        ),
    ) -> models::loadable_meta_item::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_meta_item::Content::Ready(ready.to_protobuf::<E>(&(
                    *ctx,
                    *streaming_server_url,
                    *library_item,
                    *watched,
                    *addon_name,
                    *meta_request,
                )))
            }
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
        (
            &Ctx,
            Option<&Url>,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    > for Loadable<Vec<Stream>, ResourceError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, addon_name, stream_request, meta_request): &(
            &Ctx,
            Option<&Url>,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    ) -> models::loadable_streams::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_streams::Content::Ready(models::Streams {
                streams: ready.to_protobuf::<E>(&(
                    Some(*ctx),
                    *streaming_server_url,
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
        (
            &Ctx,
            Option<&Url>,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    > for Loadable<Option<Stream>, ResourceError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, addon_name, stream_request, meta_request): &(
            &Ctx,
            Option<&Url>,
            &String,
            &ResourceRequest,
            Option<&ResourceRequest>,
        ),
    ) -> models::loadable_stream::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_stream::Content::Ready(models::OptionStream {
                    stream: ready.to_protobuf::<E>(&(
                        Some(*ctx),
                        *streaming_server_url,
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        addon_name: &Option<&String>,
    ) -> models::loadable_subtitles::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_subtitles::Content::Ready(models::Subtitles {
                    subtitles: ready.to_protobuf::<E>(addon_name),
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableSettings {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_settings::Content::Ready(ready.to_protobuf::<E>(&()))
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableBaseUrl {
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableCode {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_code::Content::Ready(ready.to_protobuf::<E>(&()))
            }
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableAuthKey {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_auth_key::Content::Ready(ready.to_protobuf::<E>(&()))
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

impl ToProtobuf<models::LoadableTramvai, ()> for Loadable<ResourcePath, EnvError> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableTramvai {
        let content = match &self {
            Loadable::Ready(ready) => {
                let deeplinks = MetaItemDeepLinks::from(ready).to_protobuf::<E>(&());
                models::loadable_tramvai::Deeplinks::Ready(deeplinks)
            }
            Loadable::Err(error) => models::loadable_tramvai::Deeplinks::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_tramvai::Deeplinks::Loading(models::Loading {}),
        };
        models::LoadableTramvai {
            deeplinks: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadablePlaybackDevices, ()> for Loadable<Vec<PlaybackDevice>, EnvError> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadablePlaybackDevices {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_playback_devices::Content::Ready(PlaybackDevices {
                    devices: ready.to_protobuf::<E>(&()),
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableStatistics {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_statistics::Content::Ready(ready.to_protobuf::<E>(&()))
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
    for Loadable<Vec<Descriptor>, ResourceError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::loadable_addon_catalog::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_addon_catalog::Content::Ready(models::Addons {
                    items: ready.to_protobuf::<E>(ctx),
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::loadable_descriptor::Content {
        match &self {
            Loadable::Ready(ready) => {
                models::loadable_descriptor::Content::Ready(ready.to_protobuf::<E>(ctx))
            }
            Loadable::Err(error) => models::loadable_descriptor::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_descriptor::Content::Loading(models::Loading {}),
        }
    }
}

impl ToProtobuf<models::LoadableModal, ()> for Loadable<Option<GetModalResponse>, CtxError> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableModal {
        let content = match &self {
            Loadable::Ready(ready) => models::loadable_modal::Content::Ready(LoadedModal {
                modal: ready.to_protobuf::<E>(&()),
            }),
            Loadable::Err(error) => models::loadable_modal::Content::Error(models::Error {
                message: match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                },
            }),
            Loadable::Loading => models::loadable_modal::Content::Loading(models::Loading {}),
        };
        models::LoadableModal {
            content: Some(content),
        }
    }
}

impl ToProtobuf<models::LoadableNotification, ()>
    for Loadable<Option<GetNotificationResponse>, CtxError>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::LoadableNotification {
        let content = match &self {
            Loadable::Ready(ready) => {
                models::loadable_notification::Content::Ready(LoadedNotification {
                    notification: ready.to_protobuf::<E>(&()),
                })
            }
            Loadable::Err(error) => models::loadable_notification::Content::Error(models::Error {
                message: match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                },
            }),
            Loadable::Loading => {
                models::loadable_notification::Content::Loading(models::Loading {})
            }
        };
        models::LoadableNotification {
            content: Some(content),
        }
    }
}
