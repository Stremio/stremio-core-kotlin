use stremio_core::deep_links::LibraryItemDeepLinks;
use stremio_core::types::library::LibraryItem;
use stremio_core::types::profile::Settings;
use stremio_core::types::streams::StreamsItem;
use url::Url;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::types;

impl ToProtobuf<types::LibraryItem, (Option<&StreamsItem>, Option<&Url>, &Settings)> for LibraryItem {
    fn to_protobuf(&self, args: &(Option<&StreamsItem>, Option<&Url>, &Settings)) -> types::LibraryItem {
        let deep_links = LibraryItemDeepLinks::from((self, args.0, args.1, args.2));
        types::LibraryItem {
            id: self.id.to_string(),
            r#type: self.r#type.to_string(),
            name: self.name.to_string(),
            poster: self.poster.to_protobuf(&()),
            poster_shape: self.poster_shape.to_protobuf(&()) as i32,
            state: types::LibraryItemState {
                time_offset: self.state.time_offset,
                duration: self.state.duration,
                video_id: self.state.video_id.clone(),
            },
            behavior_hints: self.behavior_hints.to_protobuf(&()),
            deep_links: types::MetaItemDeepLinks {
                meta_details_videos: deep_links.meta_details_videos,
                meta_details_streams: deep_links.meta_details_streams,
                player: deep_links.player,
            },
            progress: if self.state.time_offset > 0 && self.state.duration > 0 {
                Some(self.state.time_offset as f64 / self.state.duration as f64)
            } else {
                None
            },
            watched: self.state.times_watched > 0,
        }
    }
}
