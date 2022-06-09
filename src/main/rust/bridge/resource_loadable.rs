use stremio_core::models::common::ResourceLoadable;
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream};

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::LoadableCatalog, Ctx> for ResourceLoadable<Vec<MetaItemPreview>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableCatalog {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .and_then(|addon| {
                addon
                    .manifest
                    .catalogs
                    .iter()
                    .find(|manifest_catalog| manifest_catalog.id == self.request.path.id)
                    .map(|manifest_catalog| (addon, manifest_catalog))
            })
            .map(|(addon, manifest_catalog)| {
                let title = format!(
                    "{} - {} {}",
                    &addon.manifest.name,
                    &manifest_catalog
                        .name
                        .as_ref()
                        .unwrap_or(&manifest_catalog.id),
                    &self.request.path.r#type
                );
                models::LoadableCatalog {
                    title,
                    request: self.request.to_protobuf(&()),
                    content: self.content.to_protobuf(&self.request),
                }
            })
            .unwrap()
    }
}

impl ToProtobuf<models::LoadableMetaItem, Ctx> for &ResourceLoadable<MetaItem> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableMetaItem {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| {
                let addon_name = addon.manifest.name.to_owned();
                models::LoadableMetaItem {
                    title: addon_name.clone(),
                    request: self.request.to_protobuf(&()),
                    content: self
                        .content
                        .to_protobuf(&(Some(addon_name.to_owned()), self.request.to_owned())),
                }
            })
            .unwrap()
    }
}

impl ToProtobuf<models::LoadableStreams, (&Ctx, Option<&ResourceRequest>)>
for ResourceLoadable<Vec<Stream>>
{
    fn to_protobuf(
        &self,
        (ctx, meta_request): &(&Ctx, Option<&ResourceRequest>),
    ) -> models::LoadableStreams {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| {
                let addon_name = addon.manifest.name.to_owned();
                models::LoadableStreams {
                    title: addon_name.to_owned(),
                    request: self.request.to_protobuf(&()),
                    content: self.content.to_protobuf(&(
                        addon_name,
                        self.request.to_owned(),
                        meta_request.to_owned(),
                    )),
                }
            })
            .unwrap()
    }
}
