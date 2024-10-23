use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::types;
use stremio_core::deep_links::{ExternalPlayerLink, LibraryItemDeepLinks};

impl ToProtobuf<types::LibraryItemDeepLinks, ()> for LibraryItemDeepLinks {
    fn to_protobuf(&self, _args: &()) -> types::LibraryItemDeepLinks {
        types::LibraryItemDeepLinks {
            meta_details_videos: self.meta_details_videos.to_owned(),
            meta_details_streams: self.meta_details_streams.to_owned(),
            player: self.player.to_owned(),
            external_player: self.external_player.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<types::ExternalPlayerLink, ()> for ExternalPlayerLink {
    fn to_protobuf(&self, _args: &()) -> types::ExternalPlayerLink {
        types::ExternalPlayerLink {
            download: self.download.to_owned(),
            streaming: self.streaming.to_owned(),
        }
    }
}
