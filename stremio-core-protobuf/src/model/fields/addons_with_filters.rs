use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::model::AddonsWithFilters;
use crate::protobuf::stremio::core::models::LoadableAddonCatalog;
use crate::protobuf::stremio::core::{models, types};
use stremio_core::models::ctx::Ctx;
use stremio_core::models::installed_addons_with_filters::InstalledAddonsRequest;
use stremio_core::models::{catalog_with_filters, installed_addons_with_filters};

impl FromProtobuf<catalog_with_filters::Selected> for models::addons_with_filters::Selected {
    fn from_protobuf(&self) -> catalog_with_filters::Selected {
        catalog_with_filters::Selected {
            request: self.request.from_protobuf(),
        }
    }
}

impl FromProtobuf<installed_addons_with_filters::Selected>
    for models::addons_with_filters::Selected
{
    fn from_protobuf(&self) -> installed_addons_with_filters::Selected {
        installed_addons_with_filters::Selected {
            request: self.request.from_protobuf(),
        }
    }
}

impl ToProtobuf<types::ResourceRequest, ()> for InstalledAddonsRequest {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ResourceRequest {
        types::ResourceRequest {
            base: "".to_string(),
            path: types::ResourcePath {
                resource: "".to_string(),
                r#type: self.r#type.clone().unwrap_or_default(),
                id: "".to_string(),
                extra: vec![],
            },
        }
    }
}

impl ToProtobuf<models::addons_with_filters::Selected, ()>
    for installed_addons_with_filters::Selected
{
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::addons_with_filters::Selected {
        models::addons_with_filters::Selected {
            request: self.request.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<models::AddonsWithFilters, Ctx> for AddonsWithFilters {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::AddonsWithFilters {
        models::AddonsWithFilters {
            selected: self
                .remote_addons
                .selected
                .to_owned()
                .map(|selected| models::addons_with_filters::Selected {
                    request: selected.request.to_protobuf::<E>(&()),
                })
                .or_else(|| self.installed_addons.selected.to_protobuf::<E>(&())),
            selectable: models::addons_with_filters::Selectable {
                types: match &self.remote_addons.selected.is_some() {
                    true => self
                        .remote_addons
                        .selectable
                        .types
                        .iter()
                        .map(
                            |selectable_type| models::addons_with_filters::SelectableType {
                                r#type: selectable_type.r#type.to_string(),
                                selected: selectable_type.selected,
                                request: selectable_type.request.to_protobuf::<E>(&()),
                            },
                        )
                        .collect(),
                    false => self
                        .installed_addons
                        .selectable
                        .types
                        .iter()
                        .map(
                            |selectable_type| models::addons_with_filters::SelectableType {
                                r#type: selectable_type.r#type.clone().unwrap_or_default(),
                                selected: selectable_type.selected,
                                request: selectable_type.request.to_protobuf::<E>(&()),
                            },
                        )
                        .collect(),
                },
                catalogs: self
                    .remote_addons
                    .selectable
                    .catalogs
                    .iter()
                    .map(|catalog| models::addons_with_filters::SelectableCatalog {
                        name: catalog.catalog.to_owned(),
                        selected: catalog.selected,
                        request: catalog.request.to_protobuf::<E>(&()),
                    })
                    .chain([models::addons_with_filters::SelectableCatalog {
                        name: "Installed".to_string(),
                        selected: self.remote_addons.selected.is_none(),
                        request: InstalledAddonsRequest { r#type: None }.to_protobuf::<E>(&()),
                    }])
                    .collect(),
            },
            catalog: match &self.remote_addons.selected {
                Some(_) => self
                    .remote_addons
                    .catalog
                    .first()
                    .map(|page| page.to_protobuf::<E>(ctx)),
                None => {
                    self.installed_addons
                        .selected
                        .as_ref()
                        .map(|selected| LoadableAddonCatalog {
                            request: selected.request.to_protobuf::<E>(&()),
                            content: Some(models::loadable_addon_catalog::Content::Ready(
                                models::Addons {
                                    items: self.installed_addons.catalog.to_protobuf::<E>(ctx),
                                },
                            )),
                        })
                }
            },
        }
    }
}
