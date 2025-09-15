use inflector::Inflector;
use url::Url;

use stremio_core::{
    deep_links::DiscoverDeepLinks,
    models::{
        common::{DescriptorLoadable, ResourceLoadable},
        ctx::Ctx,
    },
    types::{
        addon::{Descriptor, ResourceRequest},
        library::LibraryItem,
        resource::{MetaItem, MetaItemPreview, Stream, Subtitles},
        watched_bitfield::WatchedBitField,
    },
};

use crate::{bridge::ToProtobuf, protobuf::stremio::core::models};

impl ToProtobuf<models::LoadablePage, Ctx> for ResourceLoadable<Vec<MetaItemPreview>> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::LoadablePage {
        let addon_and_catalog = ctx
            .profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .and_then(|addon| {
                addon
                    .manifest
                    .catalogs
                    .iter()
                    .find(|manifest_catalog| {
                        manifest_catalog.id == self.request.path.id
                            && manifest_catalog.r#type == self.request.path.r#type
                    })
                    .map(|manifest_catalog| (addon, manifest_catalog))
            });
        let title = addon_and_catalog
            .map(|(addon, manifest_catalog)| {
                format!(
                    "{} - {}",
                    &manifest_catalog
                        .name
                        .as_ref()
                        .unwrap_or(&addon.manifest.name)
                        .to_title_case(),
                    &manifest_catalog.r#type.to_title_case(),
                )
            })
            .unwrap_or_default();
        let deep_links = DiscoverDeepLinks::from(&self.request).to_protobuf::<E>(&());
        models::LoadablePage {
            title,
            addon_id: addon_and_catalog.map(|(addon, _)| addon.manifest.id.to_owned()),
            catalog_id: addon_and_catalog.map(|(_, catalog)| catalog.id.to_owned()),
            catalog_name: addon_and_catalog.and_then(|(addon, catalog)| {
                catalog
                    .name
                    .to_owned()
                    .or(Some(addon.manifest.name.to_owned()))
            }),
            catalog_type: addon_and_catalog.map(|(_, catalog)| catalog.r#type.to_owned()),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(ctx, &self.request)),
            deep_links,
        }
    }
}

impl
    ToProtobuf<
        models::LoadableMetaItem,
        (
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
        ),
    > for &ResourceLoadable<MetaItem>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, library_item, watched): &(
            &Ctx,
            Option<&Url>,
            Option<&LibraryItem>,
            Option<&WatchedBitField>,
        ),
    ) -> models::LoadableMetaItem {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableMetaItem {
            title: addon_name.to_string(),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
                *ctx,
                *streaming_server_url,
                *library_item,
                *watched,
                Some(&addon_name),
                &self.request,
            )),
        }
    }
}

impl ToProtobuf<models::LoadableStreams, (&Ctx, Option<&Url>, Option<&ResourceRequest>)>
    for ResourceLoadable<Vec<Stream>>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, meta_request): &(&Ctx, Option<&Url>, Option<&ResourceRequest>),
    ) -> models::LoadableStreams {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableStreams {
            title: addon_name.to_owned(),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
                ctx,
                *streaming_server_url,
                &addon_name,
                &self.request,
                *meta_request,
            )),
        }
    }
}

impl ToProtobuf<models::LoadableStream, (&Ctx, Option<&Url>, Option<&ResourceRequest>)>
    for ResourceLoadable<Option<Stream>>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, streaming_server_url, meta_request): &(&Ctx, Option<&Url>, Option<&ResourceRequest>),
    ) -> models::LoadableStream {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableStream {
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
                ctx,
                *streaming_server_url,
                &addon_name,
                &self.request,
                *meta_request,
            )),
        }
    }
}

impl ToProtobuf<models::LoadableSubtitles, Ctx> for ResourceLoadable<Vec<Subtitles>> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::LoadableSubtitles {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableSubtitles {
            title: addon_name.to_owned(),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(Some(&addon_name))),
        }
    }
}

impl ToProtobuf<models::LoadableAddonCatalog, Ctx> for &ResourceLoadable<Vec<Descriptor>> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::LoadableAddonCatalog {
        models::LoadableAddonCatalog {
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(ctx),
        }
    }
}

impl ToProtobuf<models::LoadableDescriptor, Ctx> for DescriptorLoadable {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::LoadableDescriptor {
        models::LoadableDescriptor {
            transport_url: self.transport_url.to_string(),
            content: Some(self.content.to_protobuf::<E>(ctx)),
        }
    }
}

fn get_addon_name(ctx: &Ctx, addon_url: &Url) -> String {
    ctx.profile
        .addons
        .iter()
        .find(|addon| &addon.transport_url == addon_url)
        .map(|addon| &addon.manifest.name)
        .cloned()
        .unwrap_or_default()
}
