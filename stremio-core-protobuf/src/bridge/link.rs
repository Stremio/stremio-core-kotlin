use stremio_core::types::resource::Link;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::types;

impl ToProtobuf<types::LinkPreview, ()> for Link {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::LinkPreview {
        types::LinkPreview {
            name: self.name.to_string(),
            category: self.category.to_string(),
        }
    }
}

impl ToProtobuf<types::Link, ()> for Link {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> types::Link {
        types::Link {
            name: self.name.to_string(),
            category: self.category.to_string(),
            url: self.url.to_string(),
        }
    }
}
