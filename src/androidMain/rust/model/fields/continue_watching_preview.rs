use stremio_core::models::continue_watching_preview::ContinueWatchingPreview;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::ContinueWatchingPreview, ()> for ContinueWatchingPreview {
    fn to_protobuf(&self, _args: &()) -> models::ContinueWatchingPreview {
        models::ContinueWatchingPreview {
            library_items: self.library_items.to_protobuf(&()),
        }
    }
}
