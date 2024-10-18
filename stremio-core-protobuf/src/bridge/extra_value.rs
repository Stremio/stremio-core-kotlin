use stremio_core::types::addon::ExtraValue;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<ExtraValue> for types::ExtraValue {
    fn from_protobuf(&self) -> ExtraValue {
        ExtraValue {
            name: self.name.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

impl ToProtobuf<types::ExtraValue, ()> for ExtraValue {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::ExtraValue {
        types::ExtraValue {
            name: self.name.to_owned(),
            value: self.value.to_owned(),
        }
    }
}
