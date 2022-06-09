use std::ops::Deref;

use stremio_core::models::common::{Loadable, ResourceError};
use stremio_core::models::link::LinkError;
use stremio_core::models::streaming_server::Settings;
use stremio_core::runtime::EnvError;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::api::{LinkAuthKey, LinkCodeResponse};
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream};
use url::Url;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::loadable_catalog::Content, ResourceRequest>
    for Loadable<Vec<MetaItemPreview>, ResourceError>
{
    fn to_protobuf(&self, request: &ResourceRequest) -> models::loadable_catalog::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_catalog::Content::Ready(models::Catalog {
                meta_items: ready.to_protobuf(request),
            }),
            Loadable::Err(error) => models::loadable_catalog::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_catalog::Content::Loading(models::Loading {}),
        }
    }
}

impl ToProtobuf<models::loadable_meta_item::Content, (Option<String>, ResourceRequest)>
    for Loadable<MetaItem, ResourceError>
{
    fn to_protobuf(
        &self,
        (addon_name, meta_request): &(Option<String>, ResourceRequest),
    ) -> models::loadable_meta_item::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_meta_item::Content::Ready(
                ready.to_protobuf(&(addon_name.to_owned(), meta_request.to_owned())),
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
        (String, ResourceRequest, Option<&ResourceRequest>),
    > for Loadable<Vec<Stream>, ResourceError>
{
    fn to_protobuf(
        &self,
        (addon_name, stream_request, meta_request): &(
            String,
            ResourceRequest,
            Option<&ResourceRequest>,
        ),
    ) -> models::loadable_streams::Content {
        match &self {
            Loadable::Ready(ready) => models::loadable_streams::Content::Ready(models::Streams {
                streams: ready.to_protobuf(&(
                    Some(addon_name.to_owned()),
                    Some(stream_request.deref()),
                    meta_request.to_owned(),
                )),
            }),
            Loadable::Err(error) => models::loadable_streams::Content::Error(models::Error {
                message: error.to_string(),
            }),
            Loadable::Loading => models::loadable_streams::Content::Loading(models::Loading {}),
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
