use stremio_core::deep_links::DiscoverDeepLinks;
use stremio_core::models::catalog_with_filters::Catalog;
use stremio_core::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::resource::MetaItemPreview;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

// impl<'a> From<stremio_core_web::model::CatalogsWithExtra<'a>> for CatalogsWithExtra {
//     fn from(model: stremio_core_web::model::CatalogsWithExtra<'a>) -> Self {
//         CatalogsWithExtra {
//             selected: model.selected,
//             catalogs: model.catalogs,
//         }
//     }
// }

// impl<'a> SerializeModel<wasm_bindgen::JsValue> for CatalogsWithExtra<'a> {
//     type Error = serde_json::Error;

//     fn serialize_model(&self) -> Result<wasm_bindgen::JsValue, Self::Error> {
//         wasm_bindgen::JsValue::try_from(self)
//     }
// }

impl FromProtobuf<Selected> for models::catalogs_with_extra::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            r#type: self.r#type.clone(),
            extra: self.extra.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::catalogs_with_extra::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::catalogs_with_extra::Selected {
        models::catalogs_with_extra::Selected {
            r#type: self.r#type.clone(),
            extra: self.extra.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::Catalog, Ctx> for Catalog<MetaItemPreview> {
    fn to_protobuf(&self, ctx: &Ctx) -> models::Catalog {
        models::Catalog {
            pages: self.iter().map(|page| page.to_protobuf(ctx)).collect(),
        }
    }
}

impl ToProtobuf<models::CatalogsWithExtra, Ctx> for CatalogsWithExtra {
    fn to_protobuf(&self, ctx: &Ctx) -> models::CatalogsWithExtra {
        models::CatalogsWithExtra {
            selected: self.selected.to_protobuf(&()),
            catalogs: self.catalogs.to_protobuf(ctx),
        }
    }
}

impl ToProtobuf<models::DiscoverDeepLinks, ()> for DiscoverDeepLinks {
    fn to_protobuf(&self, _args: &()) -> models::DiscoverDeepLinks {
        models::DiscoverDeepLinks {
            discover: self.discover.clone(),
        }
    }
}
