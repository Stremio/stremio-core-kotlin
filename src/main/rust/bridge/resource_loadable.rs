use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::models::common::{Loadable, ResourceLoadable};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::resource::{MetaItem, MetaItemPreview, Stream};

use crate::bridge::{ToProtobuf, ToProtobufAny, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;

impl<'a, U, R: TryIntoKotlin<'a, U>> TryIntoKotlin<'a, (Option<String>, U)>
for ResourceLoadable<R>
{
    #[inline]
    fn try_into_kotlin(
        &self,
        args: &(Option<String>, U),
        env: &JNIEnv<'a>,
    ) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let title = args.0.try_into_kotlin(&(), env)?.auto_local(env);
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let content = self.content.try_into_kotlin(&args.1, env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::ResourceLoadable).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::Loadable.value()
            ),
            &[
                title.as_obj().into(),
                request.as_obj().into(),
                content.as_obj().into(),
            ],
        )
    }
}

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
                    content: Some(match &self.content {
                        Loadable::Ready(ready) => {
                            models::loadable_catalog::Content::Ready(models::Catalog {
                                meta_items: ready.to_protobuf(&()),
                            })
                        }
                        Loadable::Err(error) => {
                            models::loadable_catalog::Content::Error(models::Error {
                                message: error.to_string(),
                            })
                        }
                        Loadable::Loading => {
                            models::loadable_catalog::Content::Loading(models::Loading {})
                        }
                    }),
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
                    title: addon_name,
                    request: self.request.to_protobuf(&()),
                    content: Some(match &self.content {
                        Loadable::Ready(ready) => {
                            models::loadable_meta_item::Content::Ready(ready.to_protobuf(&()))
                        }
                        Loadable::Err(error) => {
                            models::loadable_meta_item::Content::Error(models::Error {
                                message: error.to_string(),
                            })
                        }
                        Loadable::Loading => {
                            models::loadable_meta_item::Content::Loading(models::Loading {})
                        }
                    }),
                }
            })
            .unwrap()
    }
}

impl ToProtobuf<models::LoadableStreams, Ctx> for ResourceLoadable<Vec<Stream>> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::LoadableStreams {
        ctx.profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.request.base)
            .map(|addon| {
                let addon_name = addon.manifest.name.to_owned();
                models::LoadableStreams {
                    title: addon_name,
                    request: self.request.to_protobuf(&()),
                    content: Some(match &self.content {
                        Loadable::Ready(ready) => {
                            models::loadable_streams::Content::Ready(models::Streams {
                                streams: ready.to_protobuf(&()),
                            })
                        }
                        Loadable::Err(error) => {
                            models::loadable_streams::Content::Error(models::Error {
                                message: error.to_string(),
                            })
                        }
                        Loadable::Loading => {
                            models::loadable_streams::Content::Loading(models::Loading {})
                        }
                    }),
                }
            })
            .unwrap()
    }
}
