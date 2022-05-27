use crate::bridge::{ToProtobuf, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use prost::Message;
use stremio_core::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
use stremio_core::models::common::{Loadable, ResourceError};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ExtraValue;
use crate::protobuf::stremio::core;
use crate::protobuf::stremio::core::models;

impl TryFromKotlin for Selected {
    fn try_from_kotlin<'a>(selected: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let extra = env
            .call_method(selected, "getExtra", "()Ljava/util/List;", &[])?
            .l()?
            .auto_local(env);
        let extra = Vec::<ExtraValue>::try_from_kotlin(extra.as_obj(), env)?;
        Ok(Selected { extra })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Selected {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let extra = self.extra.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogsWithExtra_Selected)
                .unwrap(),
            "(Ljava/util/List;)V",
            &[extra.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, Ctx> for CatalogsWithExtra {
    fn try_into_kotlin(&self, ctx: &Ctx, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let selected = self.selected.try_into_kotlin(&(), env)?.auto_local(env);
        let catalogs = self
            .catalogs
            .iter()
            .filter(|catalog| {
                !matches!(&catalog.content, Loadable::Err(ResourceError::EmptyContent))
            })
            .filter_map(|catalog| {
                ctx.profile
                    .addons
                    .iter()
                    .find(|addon| addon.transport_url == catalog.request.base)
                    .and_then(|addon| {
                        addon
                            .manifest
                            .catalogs
                            .iter()
                            .find(|manifest_catalog| manifest_catalog.id == catalog.request.path.id)
                            .map(|manifest_catalog| (addon, manifest_catalog))
                    })
                    .map(|(addon, manifest_catalog)| (addon, manifest_catalog, catalog))
            })
            .map(|(addon, manifest_catalog, catalog)| {
                let title = format!(
                    "{} - {} {}",
                    &addon.manifest.name,
                    &manifest_catalog
                        .name
                        .as_ref()
                        .unwrap_or(&manifest_catalog.id),
                    &catalog.request.path.r#type
                );
                catalog.try_into_kotlin(&(Some(title), ()), env)
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::CatalogsWithExtra).unwrap(),
            format!(
                "(L{};L{};)V",
                &KotlinClassName::CatalogsWithExtra_Selected.value(),
                "java/util/List"
            ),
            &[selected.as_obj().into(), catalogs.as_obj().into()],
        )
    }
}
impl ToProtobuf<Ctx> for CatalogsWithExtra {
    fn to_protobuf(&self, args: &Ctx) -> Vec<u8> {
        let selected = self.selected.map(|s|
            models::catalogs_with_extra::Selected {
                extra: vec![]
            }
        );
        let catalogs_with_extra = models::CatalogsWithExtra {
            selected,
            catalogs: vec![]
        };
        return catalogs_with_extra.encode_to_vec();
    }
}
