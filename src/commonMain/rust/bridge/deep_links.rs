use stremio_core::deep_links::{DiscoverDeepLinks, LibraryDeepLinks};

use crate::protobuf::stremio::core::deep_links;

use super::ToProtobuf;

impl ToProtobuf<deep_links::DiscoverDeepLinks, ()> for DiscoverDeepLinks {
    fn to_protobuf(&self, _args: &()) -> deep_links::DiscoverDeepLinks {
        deep_links::DiscoverDeepLinks {
            discover: self.discover.clone(),
        }
    }
}

impl ToProtobuf<deep_links::LibraryDeepLinks, ()> for LibraryDeepLinks {
    fn to_protobuf(&self, _args: &()) -> deep_links::LibraryDeepLinks {
        deep_links::LibraryDeepLinks {
            library: self.library.clone(),
        }
    }
}