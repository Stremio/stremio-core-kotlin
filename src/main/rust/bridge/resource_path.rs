use stremio_core::types::addon::ResourcePath;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<ResourcePath> for types::ResourcePath {
    fn from_protobuf(&self) -> ResourcePath {
        ResourcePath {
            resource: self.resource.to_owned(),
            r#type: self.r#type.to_owned(),
            id: self.id.to_owned(),
            extra: self.extra.from_protobuf(),
        }
    }
}

impl ToProtobuf<types::ResourcePath, ()> for ResourcePath {
    fn to_protobuf(&self, _args: &()) -> types::ResourcePath {
        types::ResourcePath {
            resource: self.resource.to_owned(),
            r#type: self.r#type.to_owned(),
            id: self.id.to_owned(),
            extra: self.extra.to_protobuf(&()),
        }
    }
}
