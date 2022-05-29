use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::catalog_with_filters::{
    CatalogWithFilters, Selectable, SelectableCatalog, SelectableExtra, SelectableExtraOption,
    SelectablePage, SelectableType, Selected,
};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::MetaItemPreview;
use stremio_deeplinks::DiscoverDeepLinks;

impl TryFromKotlin for Selected {
    fn try_from_kotlin<'a>(selected: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let request = env
            .call_method(
                selected,
                "getRequest",
                format!("()L{};", KotlinClassName::ResourceRequest.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let request = ResourceRequest::try_from_kotlin(request.as_obj(), env)?;
        Ok(Selected { request })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for DiscoverDeepLinks {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let discover = self.discover.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::DiscoverDeepLinks).unwrap(),
            format!("(L{};)V", "java/lang/String"),
            &[discover.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SelectablePage {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let deep_links = DiscoverDeepLinks::from(&self.request)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_SelectablePage)
                .unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::DiscoverDeepLinks.value()
            ),
            &[request.as_obj().into(), deep_links.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SelectableExtraOption {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let value = self.value.try_into_kotlin(&(), env)?.auto_local(env);
        let selected = self.selected.into();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let deep_links = DiscoverDeepLinks::from(&self.request)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_SelectableExtraOption)
                .unwrap(),
            format!(
                "(L{};ZL{};L{};)V",
                "java/lang/String",
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::DiscoverDeepLinks.value()
            ),
            &[
                value.as_obj().into(),
                selected,
                request.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SelectableExtra {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let is_required = self.is_required.into();
        let options = self.options.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_SelectableExtra)
                .unwrap(),
            format!("(L{};ZL{};)V", "java/lang/String", "java/util/List"),
            &[name.as_obj().into(), is_required, options.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SelectableType {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let selected = self.selected.into();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let deep_links = DiscoverDeepLinks::from(&self.request)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_SelectableType)
                .unwrap(),
            format!(
                "(L{};ZL{};L{};)V",
                "java/lang/String",
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::DiscoverDeepLinks.value()
            ),
            &[
                r#type.as_obj().into(),
                selected,
                request.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for SelectableCatalog {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let name = self.catalog.try_into_kotlin(&(), env)?.auto_local(env);
        let selected = self.selected.into();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let deep_links = DiscoverDeepLinks::from(&self.request)
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_SelectableCatalog)
                .unwrap(),
            format!(
                "(L{};ZL{};L{};)V",
                "java/lang/String",
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::DiscoverDeepLinks.value()
            ),
            &[
                name.as_obj().into(),
                selected,
                request.as_obj().into(),
                deep_links.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Selectable {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let types = self.types.try_into_kotlin(&(), env)?.auto_local(env);
        let catalogs = self.catalogs.try_into_kotlin(&(), env)?.auto_local(env);
        let extra = self.extra.try_into_kotlin(&(), env)?.auto_local(env);
        let prev_page = self.prev_page.try_into_kotlin(&(), env)?.auto_local(env);
        let next_page = self.next_page.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_Selectable)
                .unwrap(),
            format!(
                "(L{};L{};L{};L{};L{};)V",
                "java/util/List",
                "java/util/List",
                "java/util/List",
                KotlinClassName::CatalogWithFilters_SelectablePage.value(),
                KotlinClassName::CatalogWithFilters_SelectablePage.value()
            ),
            &[
                types.as_obj().into(),
                catalogs.as_obj().into(),
                extra.as_obj().into(),
                prev_page.as_obj().into(),
                next_page.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Selected {
    fn try_into_kotlin(&self, _: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::CatalogWithFilters_Selected)
                .unwrap(),
            format!("(L{};)V", KotlinClassName::ResourceRequest.value()),
            &[request.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, Ctx> for CatalogWithFilters<MetaItemPreview> {
    fn try_into_kotlin(&self, _ctx: &Ctx, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let selected = self.selected.try_into_kotlin(&(), env)?.auto_local(env);
        let selectable = self.selectable.try_into_kotlin(&(), env)?.auto_local(env);
        let catalog = self
            .catalog
            .try_into_kotlin(&(None, ()), env)?
            .auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::CatalogWithFilters).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::CatalogWithFilters_Selected.value(),
                KotlinClassName::CatalogWithFilters_Selectable.value(),
                KotlinClassName::ResourceLoadable.value()
            ),
            &[
                selected.as_obj().into(),
                selectable.as_obj().into(),
                catalog.as_obj().into(),
            ],
        )
    }
}

impl ToProtobuf<models::catalog_with_filters::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::Selected {
        models::catalog_with_filters::Selected {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::SelectableType, ()> for SelectableType {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::SelectableType {
        models::catalog_with_filters::SelectableType {
            r#type: self.r#type.to_string(),
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::SelectableCatalog, ()> for SelectableCatalog {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::SelectableCatalog {
        models::catalog_with_filters::SelectableCatalog {
            name: self.catalog.to_string(),
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::SelectableExtraOption, ()> for SelectableExtraOption {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::SelectableExtraOption {
        models::catalog_with_filters::SelectableExtraOption {
            value: self.value.clone(),
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::SelectableExtra, ()> for SelectableExtra {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::SelectableExtra {
        models::catalog_with_filters::SelectableExtra {
            name: self.name.to_string(),
            is_required: self.is_required,
            options: self.options.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::SelectablePage, ()> for SelectablePage {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::SelectablePage {
        models::catalog_with_filters::SelectablePage {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::catalog_with_filters::Selectable, ()> for Selectable {
    fn to_protobuf(&self, _args: &()) -> models::catalog_with_filters::Selectable {
        models::catalog_with_filters::Selectable {
            types: self.types.to_protobuf(&()),
            catalogs: self.catalogs.to_protobuf(&()),
            extra: self.extra.to_protobuf(&()),
            prev_page: self.prev_page.to_protobuf(&()),
            next_page: self.next_page.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::CatalogWithFilters, Ctx> for CatalogWithFilters<MetaItemPreview> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::CatalogWithFilters {
        models::CatalogWithFilters {
            selected: self.selected.to_protobuf(&()),
            selectable: self.selectable.to_protobuf(&()),
            catalog: self.catalog.to_protobuf(ctx),
        }
    }
}
