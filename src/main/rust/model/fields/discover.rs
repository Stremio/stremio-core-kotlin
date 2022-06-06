use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::catalog_with_filters::{
    CatalogWithFilters, Selectable, SelectableCatalog, SelectableExtra, SelectableExtraOption,
    SelectablePage, SelectableType, Selected,
};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::MetaItemPreview;

use crate::bridge::{ToProtobuf, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;

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
