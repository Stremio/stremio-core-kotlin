use stremio_core::types::library::LibraryItem;
use stremio_deeplinks::LibraryItemDeepLinks;

use crate::bridge::{ToProtobuf, ToProtobufAny};
use crate::protobuf::stremio::core::types;

impl ToProtobuf<types::LibraryItem, ()> for LibraryItem {
    fn to_protobuf(&self, _args: &()) -> types::LibraryItem {
        let deep_links = LibraryItemDeepLinks::from(self);
        types::LibraryItem {
            id: self.id.to_string(),
            r#type: self.r#type.to_string(),
            name: self.name.to_string(),
            poster: self.poster.clone(),
            poster_shape: self.poster_shape.to_protobuf(&()) as i32,
            state: types::LibraryItemState {
                time_offset: self.state.time_offset,
                duration: self.state.duration,
            },
            behavior_hints: types::MetaItemBehaviorHints {
                default_video_id: self.behavior_hints.default_video_id.clone(),
                featured_video_id: None,
                has_scheduled_videos: false,
            },
            deep_links: types::MetaItemDeepLinks {
                meta_details_videos: deep_links.meta_details_videos,
                meta_details_streams: deep_links.meta_details_streams,
                player: deep_links.player,
            },
        }
    }
}
