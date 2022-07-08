use std::convert::identity;
use stremio_core::models::catalog_with_filters::Catalog;
use stremio_core::models::catalogs_with_extra::{CatalogPage, CatalogsWithExtra, Selected};
use stremio_core::models::common::Loadable;
use stremio_core::models::ctx::Ctx;
use stremio_core::types::resource::MetaItemPreview;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

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

impl ToProtobuf<Option<models::LoadableCatalog>, Ctx> for Catalog<MetaItemPreview> {
    fn to_protobuf(&self, ctx: &Ctx) -> Option<models::LoadableCatalog> {
        self.first()
            .map(|first_page| match first_page {
                CatalogPage {
                    content: Some(Loadable::Ready(_)),
                    ..
                } => CatalogPage {
                    content: Some(Loadable::Ready(
                        self.iter()
                            .filter_map(|page| page.content.as_ref())
                            .filter_map(|content| content.ready())
                            .flatten()
                            .map(|item| item.to_owned())
                            .collect(),
                    )),
                    ..first_page.to_owned()
                },
                _ => first_page.to_owned(),
            })
            .map(|first_page| first_page.to_protobuf(ctx))
    }
}

impl ToProtobuf<models::CatalogsWithExtra, Ctx> for CatalogsWithExtra {
    fn to_protobuf(&self, ctx: &Ctx) -> models::CatalogsWithExtra {
        models::CatalogsWithExtra {
            selected: self.selected.to_protobuf(&()),
            catalogs: self
                .catalogs
                .to_protobuf(ctx)
                .into_iter()
                .filter_map(identity)
                .collect(),
        }
    }
}
