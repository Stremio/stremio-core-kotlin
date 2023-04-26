use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;
use stremio_core::models::addon_details::{AddonDetails, Selected};

impl FromProtobuf<Selected> for models::addon_details::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            transport_url: self.transport_url.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::addon_details::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::addon_details::Selected {
        models::addon_details::Selected {
            transport_url: self.transport_url.to_string(),
        }
    }
}

impl ToProtobuf<models::AddonDetails, ()> for AddonDetails {
    fn to_protobuf(&self, _args: &()) -> models::AddonDetails {
        models::AddonDetails {
            selected: self.selected.to_protobuf(&()),
            local_addon: self.local_addon.to_protobuf(&()),
            remote_addon: self.remote_addon.to_protobuf(&()),
        }
    }
}
