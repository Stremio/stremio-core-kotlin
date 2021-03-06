use stremio_core::types::addon::ResourceRequest;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<ResourceRequest> for types::ResourceRequest {
    fn from_protobuf(&self) -> ResourceRequest {
        ResourceRequest {
            base: self.base.from_protobuf(),
            path: self.path.from_protobuf(),
        }
    }
}

impl ToProtobuf<types::ResourceRequest, ()> for ResourceRequest {
    fn to_protobuf(&self, _args: &()) -> types::ResourceRequest {
        types::ResourceRequest {
            base: self.base.to_string(),
            path: self.path.to_protobuf(&()),
        }
    }
}
