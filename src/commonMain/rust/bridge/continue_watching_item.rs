use stremio_core::deep_links::LibraryItemDeepLinks;
use stremio_core::models::continue_watching_preview::Item;
use stremio_core::types::profile::Settings;
use stremio_core::types::streams::StreamsItem;
use url::Url;

use crate::protobuf::stremio::core::models;
use crate::protobuf::stremio::core::types;

use super::ToProtobuf;

impl ToProtobuf<models::ContinueWatchingItem, (Option<&StreamsItem>, Option<&Url>, &Settings)>
    for Item
{
    fn to_protobuf(
        &self,
        (streams_item, streaming_server_url, settings): &(
            Option<&StreamsItem>,
            Option<&Url>,
            &Settings,
        ),
    ) -> models::ContinueWatchingItem {
        let deep_links = LibraryItemDeepLinks::from((
            &self.library_item,
            *streams_item,
            *streaming_server_url,
            *settings,
        ));
        models::ContinueWatchingItem {
            id: self.library_item.id.to_string(),
            r#type: self.library_item.r#type.to_string(),
            name: self.library_item.name.to_string(),
            poster: self.library_item.poster.to_protobuf(&()),
            poster_shape: self.library_item.poster_shape.to_protobuf(&()) as i32,
            state: types::LibraryItemState {
                time_offset: self.library_item.state.time_offset,
                duration: self.library_item.state.duration,
                video_id: self.library_item.state.video_id.clone(),
            },
            behavior_hints: self.library_item.behavior_hints.to_protobuf(&()),
            deep_links: types::MetaItemDeepLinks {
                meta_details_videos: deep_links.meta_details_videos,
                meta_details_streams: deep_links.meta_details_streams,
                player: deep_links.player,
            },
            progress: if self.library_item.state.time_offset > 0
                && self.library_item.state.duration > 0
            {
                Some(
                    self.library_item.state.time_offset as f64
                        / self.library_item.state.duration as f64,
                )
            } else {
                None
            },
            watched: self.library_item.state.times_watched > 0,
            notifications: self.notifications as u64,
        }
    }
}
