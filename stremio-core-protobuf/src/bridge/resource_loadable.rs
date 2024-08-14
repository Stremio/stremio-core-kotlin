use inflector::Inflector;
use url::Url;

use stremio_core::deep_links::DiscoverDeepLinks;
use stremio_core::models::common::{DescriptorLoadable, ResourceLoadable};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::{DescriptorPreview, ResourceRequest};
use stremio_core::types::library::LibraryItem;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream, Subtitles};
use stremio_core::types::watched_bitfield::WatchedBitField;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;
// use crate::stremio_core_models::types::ResourcePath;

// impl<'a> ToProtobuf<models::LoadablePage, Ctx> for stremio_core_web::model::serialize_catalogs_with_extra::ResourceLoadable<'a> {
//     fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, ctx: &Ctx) -> models::LoadablePage {
//         let deep_links = DiscoverDeepLinks::from(&self.request).to_protobuf::<E>(&());
//         models::LoadablePage {
//             title: self.name.to_string(),
//             request: crate::protobuf::stremio::core::types::ResourceRequest {
//                 base: "todo".into(),
//                 path: ResourcePath {
//                     resource: "todo".to_string(),
//                     r#type: "todo".into(),
//                     id: "todo".into(),
//                     extra: vec![],
//                 },
//             },
//             content: self.content.to_protobuf::<E>(&(ctx, &self.request)),
//             deep_links,
//         }
//     }
// }

impl ToProtobuf<models::LoadablePage, Ctx> for ResourceLoadable<Vec<MetaItemPreview>> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::LoadablePage {
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
                    .find(|manifest_catalog| {
                        manifest_catalog.id == self.request.path.id
                            && manifest_catalog.r#type == self.request.path.r#type
                    })
                    .map(|manifest_catalog| (addon, manifest_catalog))
            })
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
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(ctx, &self.request)),
            deep_links,
        }
    }
}

impl ToProtobuf<models::LoadableMetaItem, (&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>)>
    for &ResourceLoadable<MetaItem>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, library_item, watched): &(&Ctx, Option<&LibraryItem>, Option<&WatchedBitField>),
    ) -> models::LoadableMetaItem {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableMetaItem {
            title: addon_name.to_string(),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, meta_request): &(&Ctx, Option<&ResourceRequest>),
    ) -> models::LoadableStreams {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableStreams {
            title: addon_name.to_owned(),
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
                ctx,
                &addon_name,
                &self.request,
                *meta_request,
            )),
        }
    }
}

impl ToProtobuf<models::LoadableStream, (&Ctx, Option<&ResourceRequest>)>
    for ResourceLoadable<Option<Stream>>
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        (ctx, meta_request): &(&Ctx, Option<&ResourceRequest>),
    ) -> models::LoadableStream {
        let addon_name = get_addon_name(ctx, &self.request.base);
        models::LoadableStream {
            request: self.request.to_protobuf::<E>(&()),
            content: self.content.to_protobuf::<E>(&(
                ctx,
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

impl ToProtobuf<models::LoadableAddonCatalog, Ctx> for &ResourceLoadable<Vec<DescriptorPreview>> {
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
