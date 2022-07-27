use stremio_core::models::common::ResourceLoadable;
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::library::LibraryItem;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles};
use stremio_watched_bitfield::WatchedBitField;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::LoadablePage, Ctx> for ResourceLoadable<Vec<MetaItemPreview>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadablePage {
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
                models::LoadablePage {
                    title,
                    request: self.request.to_protobuf(&()),
                    content: self.content.to_protobuf(&(ctx, &self.request)),
                }
            })
            .unwrap()
    }
}

impl ToProtobuf<models::LoadableMetaItem, (&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>)>
    for &ResourceLoadable<MetaItem>
{
    fn to_protobuf(
        &self,
        (ctx, library_item, watched): &(&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>),
    ) -> models::LoadableMetaItem {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| {
                let addon_name = &addon.manifest.name;
                models::LoadableMetaItem {
                    title: addon_name.to_string(),
                    request: self.request.to_protobuf(&()),
                    content: self.content.to_protobuf(&(
                        *library_item,
                        *watched,
                        Some(addon_name),
                        &self.request,
                    )),
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
                let addon_name = &addon.manifest.name;
                models::LoadableStreams {
                    title: addon_name.to_string(),
                    request: self.request.to_protobuf(&()),
                    content: self
                        .content
                        .to_protobuf(&(addon_name, &self.request, *meta_request)),
                }
            })
            .unwrap()
    }
}

impl ToProtobuf<models::LoadableSubtitles, Ctx> for ResourceLoadable<Vec<Subtitles>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableSubtitles {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| {
                let addon_name = addon.manifest.name.to_owned();
                models::LoadableSubtitles {
                    title: addon_name,
                    request: self.request.to_protobuf(&()),
                    content: self.content.to_protobuf(&()),
                }
            })
            .unwrap()
    }
}
