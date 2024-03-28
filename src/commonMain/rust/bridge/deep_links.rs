use stremio_core::deep_links::{DiscoverDeepLinks, LibraryDeepLinks};

use crate::protobuf::stremio::core::deeplinks;

use super::ToProtobuf;

impl ToProtobuf<deeplinks::DiscoverDeepLinks, ()> for DiscoverDeepLinks {
    fn to_protobuf(&self, _args: &()) -> deeplinks::DiscoverDeepLinks {
        deeplinks::DiscoverDeepLinks {
            discover: self.discover.clone(),
        }
    }
}

impl ToProtobuf<deeplinks::LibraryDeepLinks, ()> for LibraryDeepLinks {
    fn to_protobuf(&self, _args: &()) -> deeplinks::LibraryDeepLinks {
        deeplinks::LibraryDeepLinks {
            library: self.library.clone(),
        }
    }
}
