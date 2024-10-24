use stremio_core::deep_links::DiscoverDeepLinks;
use stremio_core::models::catalog_with_filters::Catalog;
use stremio_core::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::catalogs_with_extra::Selected {
        models::catalogs_with_extra::Selected {
            r#type: self.r#type.clone(),
            extra: self.extra.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<models::Catalog, Ctx> for Catalog<MetaItemPreview> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, ctx: &Ctx) -> models::Catalog {
        models::Catalog {
            pages: self.iter().map(|page| page.to_protobuf::<E>(ctx)).collect(),
        }
    }
}

impl ToProtobuf<models::CatalogsWithExtra, Ctx> for CatalogsWithExtra {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::CatalogsWithExtra {
        models::CatalogsWithExtra {
            selected: self.selected.to_protobuf::<E>(&()),
            catalogs: self.catalogs.to_protobuf::<E>(ctx),
        }
    }
}

impl ToProtobuf<models::DiscoverDeepLinks, ()> for DiscoverDeepLinks {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::DiscoverDeepLinks {
        models::DiscoverDeepLinks {
            discover: self.discover.clone(),
        }
    }
}
