use stremio_core::models::common::{DescriptorLoadable, ResourceLoadable};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::{DescriptorPreview, ResourceRequest};
use stremio_core::types::library::LibraryItem;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles};
use stremio_watched_bitfield::WatchedBitField;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::LoadablePage, Ctx> for ResourceLoadable<Vec<MetaItemPreview>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadablePage {
        let title = ctx
            .profile
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
                format!(
                    "{} - {} {}",
                    &addon.manifest.name,
                    &manifest_catalog
                        .name
                        .as_ref()
                        .unwrap_or(&manifest_catalog.id),
                    &self.request.path.r#type
                )
            })
            .unwrap_or_default();
        models::LoadablePage {
            title,
            request: self.request.to_protobuf(&()),
            content: self.content.to_protobuf(&(ctx, &self.request)),
        }
    }
}

impl ToProtobuf<models::LoadableMetaItem, (&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>)>
    for &ResourceLoadable<MetaItem>
{
    fn to_protobuf(
        &self,
        (ctx, library_item, watched): &(&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>),
    ) -> models::LoadableMetaItem {
        let addon_name = ctx
            .profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| &addon.manifest.name)
            .cloned()
            .unwrap_or_default();
        models::LoadableMetaItem {
            title: addon_name.to_string(),
            request: self.request.to_protobuf(&()),
            content: self.content.to_protobuf(&(
                *library_item,
                *watched,
                Some(&addon_name),
                &self.request,
            )),
        }
    }
}

impl ToProtobuf<models::LoadableStreams, (&Ctx, Option<&ResourceRequest>)>
    for ResourceLoadable<Vec<Stream>>
{
    fn to_protobuf(
        &self,
        (ctx, meta_request): &(&Ctx, Option<&ResourceRequest>),
    ) -> models::LoadableStreams {
        let addon_name = ctx
            .profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| &addon.manifest.name)
            .cloned()
            .unwrap_or_default();
        models::LoadableStreams {
            title: addon_name.to_owned(),
            request: self.request.to_protobuf(&()),
            content: self
                .content
                .to_protobuf(&(&addon_name, &self.request, *meta_request)),
        }
    }
}

impl ToProtobuf<models::LoadableSubtitles, Ctx> for ResourceLoadable<Vec<Subtitles>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableSubtitles {
        let addon_name = ctx
            .profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| &addon.manifest.name)
            .cloned()
            .unwrap_or_default();
        models::LoadableSubtitles {
            title: addon_name.to_owned(),
            request: self.request.to_protobuf(&()),
            content: self.content.to_protobuf(&(Some(&addon_name))),
        }
    }
}

impl ToProtobuf<models::LoadableAddonCatalog, Ctx> for &ResourceLoadable<Vec<DescriptorPreview>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableAddonCatalog {
        models::LoadableAddonCatalog {
            request: self.request.to_protobuf(&()),
            content: self.content.to_protobuf(ctx),
        }
    }
}

impl ToProtobuf<models::LoadableDescriptor, ()> for DescriptorLoadable {
    fn to_protobuf(&self, _args: &()) -> models::LoadableDescriptor {
        models::LoadableDescriptor {
            transport_url: self.transport_url.to_string(),
            content: Some(self.content.to_protobuf(&())),
        }
    }
}
