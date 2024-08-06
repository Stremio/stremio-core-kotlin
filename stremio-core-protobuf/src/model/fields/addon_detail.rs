use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;
use stremio_core::models::addon_details::{AddonDetails, Selected};
use stremio_core::models::ctx::Ctx;

impl FromProtobuf<Selected> for models::addon_details::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            transport_url: self.transport_url.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::addon_details::Selected, ()> for Selected {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> models::addon_details::Selected {
        models::addon_details::Selected {
            transport_url: self.transport_url.to_string(),
        }
    }
}

impl ToProtobuf<models::AddonDetails, Ctx> for AddonDetails {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, ctx: &Ctx) -> models::AddonDetails {
        models::AddonDetails {
            selected: self.selected.to_protobuf::<E>(&()),
            local_addon: self.local_addon.to_protobuf::<E>(ctx),
            remote_addon: self.remote_addon.to_protobuf::<E>(ctx),
        }
    }
}
