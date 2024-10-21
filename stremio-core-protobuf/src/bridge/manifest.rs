use std::str::FromStr;

use semver::Version;

use stremio_core::{
    models::ctx::Ctx,
    types::addon::{
        Descriptor, DescriptorFlags, DescriptorPreview, ExtraProp, Manifest, ManifestBehaviorHints,
        ManifestCatalog, ManifestExtra, ManifestPreview, ManifestResource, OptionsLimit,
    },
};

use crate::{
    bridge::{FromProtobuf, ToProtobuf},
    protobuf::stremio::core::types,
};

impl FromProtobuf<ExtraProp> for types::ExtraProp {
    fn from_protobuf(&self) -> ExtraProp {
        ExtraProp {
            name: self.name.to_owned(),
            is_required: self.is_required,
            options: self.options.to_owned(),
            options_limit: OptionsLimit(self.options_limit as usize),
        }
    }
}

impl FromProtobuf<ManifestBehaviorHints> for types::ManifestBehaviorHints {
    fn from_protobuf(&self) -> ManifestBehaviorHints {
        ManifestBehaviorHints {
            adult: self.adult,
            p2p: self.p2p,
            configurable: self.configurable,
            configuration_required: self.configuration_required,
        }
    }
}

impl FromProtobuf<ManifestExtra> for types::manifest_extra::Extra {
    fn from_protobuf(&self) -> ManifestExtra {
        match self {
            types::manifest_extra::Extra::Full(extra) => ManifestExtra::Full {
                props: extra.props.from_protobuf(),
            },
            types::manifest_extra::Extra::Short(extra) => ManifestExtra::Short {
                required: extra.required.to_owned(),
                supported: extra.supported.to_owned(),
            },
        }
    }
}

impl FromProtobuf<ManifestCatalog> for types::ManifestCatalog {
    fn from_protobuf(&self) -> ManifestCatalog {
        ManifestCatalog {
            id: self.id.to_owned(),
            r#type: self.r#type.to_owned(),
            name: self.name.clone(),
            extra: self.extra.extra.from_protobuf().unwrap(),
        }
    }
}

impl FromProtobuf<ManifestResource> for types::ManifestResource {
    fn from_protobuf(&self) -> ManifestResource {
        if self.types.is_empty() && self.id_prefixes.is_empty() {
            ManifestResource::Short(self.name.to_owned())
        } else {
            ManifestResource::Full {
                name: self.name.to_owned(),
                types: Some(self.types.to_owned()),
                id_prefixes: Some(self.id_prefixes.to_owned()),
            }
        }
    }
}

impl FromProtobuf<ManifestPreview> for types::ManifestPreview {
    fn from_protobuf(&self) -> ManifestPreview {
        ManifestPreview {
            id: self.id.to_owned(),
            version: Version::from_str(self.version.as_str())
                .expect("Manifest.version parsing failed"),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            logo: self.logo.from_protobuf(),
            background: self.background.from_protobuf(),
            types: self.types.to_owned(),
            behavior_hints: self.behavior_hints.from_protobuf(),
        }
    }
}

impl FromProtobuf<Manifest> for types::Manifest {
    fn from_protobuf(&self) -> Manifest {
        Manifest {
            id: self.id.to_owned(),
            version: Version::from_str(self.version.as_str())
                .expect("Manifest.version parsing failed"),
            name: self.name.to_owned(),
            contact_email: None,
            description: self.description.to_owned(),
            logo: self.logo.from_protobuf(),
            background: self.background.from_protobuf(),
            types: self.types.to_owned(),
            resources: self.resources.from_protobuf(),
            id_prefixes: Some(self.id_prefixes.to_owned())
                .filter(|id_prefixes| !id_prefixes.is_empty()),
            catalogs: self.catalogs.from_protobuf(),
            addon_catalogs: self.addon_catalogs.from_protobuf(),
            behavior_hints: self.behavior_hints.from_protobuf(),
        }
    }
}

impl FromProtobuf<DescriptorFlags> for types::DescriptorFlags {
    fn from_protobuf(&self) -> DescriptorFlags {
        DescriptorFlags {
            official: self.official,
            protected: self.protected,
        }
    }
}

impl FromProtobuf<Descriptor> for types::AddonDescriptor {
    fn from_protobuf(&self) -> Descriptor {
        Descriptor {
            manifest: self.manifest.from_protobuf(),
            transport_url: self.transport_url.from_protobuf(),
            flags: self.flags.from_protobuf(),
        }
    }
}

impl ToProtobuf<types::ExtraProp, ()> for ExtraProp {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::ExtraProp {
        types::ExtraProp {
            name: self.name.to_owned(),
            is_required: self.is_required,
            options: self.options.to_owned(),
            options_limit: self.options_limit.0 as i32,
        }
    }
}

impl ToProtobuf<types::ManifestBehaviorHints, ()> for ManifestBehaviorHints {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ManifestBehaviorHints {
        types::ManifestBehaviorHints {
            adult: self.adult,
            p2p: self.p2p,
            configurable: self.configurable,
            configuration_required: self.configuration_required,
        }
    }
}

impl ToProtobuf<types::manifest_extra::Extra, ()> for ManifestExtra {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::manifest_extra::Extra {
        match self {
            ManifestExtra::Full { props } => {
                types::manifest_extra::Extra::Full(types::FullManifestExtra {
                    props: props.to_protobuf::<E>(&()),
                })
            }
            ManifestExtra::Short {
                required,
                supported,
            } => types::manifest_extra::Extra::Short(types::ShortManifestExtra {
                required: required.to_owned(),
                supported: supported.to_owned(),
            }),
        }
    }
}

impl ToProtobuf<types::ManifestCatalog, ()> for ManifestCatalog {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ManifestCatalog {
        types::ManifestCatalog {
            id: self.id.to_owned(),
            r#type: self.r#type.to_owned(),
            name: self.name.clone(),
            extra: types::ManifestExtra {
                extra: Some(self.extra.to_protobuf::<E>(&())),
            },
        }
    }
}

impl ToProtobuf<types::ManifestResource, ()> for ManifestResource {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ManifestResource {
        match self {
            ManifestResource::Short(name) => types::ManifestResource {
                name: name.to_owned(),
                types: vec![],
                id_prefixes: vec![],
            },
            ManifestResource::Full {
                name,
                types,
                id_prefixes,
            } => types::ManifestResource {
                name: name.to_owned(),
                types: types.to_owned().unwrap_or_default(),
                id_prefixes: id_prefixes.to_owned().unwrap_or_default(),
            },
        }
    }
}

impl ToProtobuf<types::ManifestPreview, ()> for ManifestPreview {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ManifestPreview {
        types::ManifestPreview {
            id: self.id.to_owned(),
            version: self.version.to_string(),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            logo: self.logo.to_protobuf::<E>(&()),
            background: self.background.to_protobuf::<E>(&()),
            types: self.types.to_owned(),
            behavior_hints: self.behavior_hints.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<types::Manifest, ()> for Manifest {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::Manifest {
        types::Manifest {
            id: self.id.to_owned(),
            version: self.version.to_string(),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            logo: self.logo.to_protobuf::<E>(&()),
            background: self.background.to_protobuf::<E>(&()),
            types: self.types.to_owned(),
            contact_email: self.contact_email.to_owned(),
            resources: self.resources.to_protobuf::<E>(&()),
            id_prefixes: self.id_prefixes.to_owned().unwrap_or_default(),
            catalogs: self.catalogs.to_protobuf::<E>(&()),
            addon_catalogs: self.addon_catalogs.to_protobuf::<E>(&()),
            behavior_hints: self.behavior_hints.to_protobuf::<E>(&()),
        }
    }
}

impl ToProtobuf<types::DescriptorFlags, ()> for DescriptorFlags {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::DescriptorFlags {
        types::DescriptorFlags {
            official: self.official,
            protected: self.protected,
        }
    }
}

impl ToProtobuf<types::DescriptorPreview, Ctx> for DescriptorPreview {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> types::DescriptorPreview {
        types::DescriptorPreview {
            manifest: self.manifest.to_protobuf::<E>(&()),
            transport_url: self.transport_url.to_protobuf::<E>(&()),
            installed: ctx
                .profile
                .addons
                .iter()
                .any(|addon| addon.transport_url == self.transport_url),
        }
    }
}

impl ToProtobuf<types::AddonDescriptor, Ctx> for Descriptor {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> types::AddonDescriptor {
        let installed_addon = ctx
            .profile
            .addons
            .iter()
            .find(|addon| addon.transport_url == self.transport_url);
        types::AddonDescriptor {
            manifest: self.manifest.to_protobuf::<E>(&()),
            transport_url: self.transport_url.to_protobuf::<E>(&()),
            flags: self.flags.to_protobuf::<E>(&()),
            installed: installed_addon.is_some(),
            installable: installed_addon.is_none()
                && !self.manifest.behavior_hints.configuration_required,
            upgradeable: installed_addon
                .filter(|addon| !addon.flags.protected)
                .map(|addon| addon.manifest.version != self.manifest.version)
                .unwrap_or_default(),
            uninstallable: installed_addon
                .filter(|addon| !addon.flags.protected)
                .is_some(),
        }
    }
}
