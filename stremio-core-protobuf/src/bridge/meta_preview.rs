use chrono::Duration;

use stremio_core::deep_links::MetaItemDeepLinks;
use stremio_core::models::ctx::Ctx;
use stremio_core::runtime::Env;
use stremio_core::types::addon::ResourceRequest;
use stremio_core::types::resource::{MetaItemBehaviorHints, MetaItemPreview, PosterShape};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<MetaItemBehaviorHints> for types::MetaItemBehaviorHints {
    fn from_protobuf(&self) -> MetaItemBehaviorHints {
        MetaItemBehaviorHints {
            default_video_id: self.default_video_id.to_owned(),
            featured_video_id: self.featured_video_id.to_owned(),
            has_scheduled_videos: self.has_scheduled_videos,
            other: Default::default(),
        }
    }
}

impl FromProtobuf<MetaItemPreview> for types::MetaItemPreview {
    fn from_protobuf(&self) -> MetaItemPreview {
        MetaItemPreview {
            id: self.id.to_owned(),
            r#type: self.r#type.to_owned(),
            name: self.name.to_owned(),
            poster_shape: types::PosterShape::try_from(self.poster_shape)
                .ok()
                .from_protobuf()
                .unwrap_or(PosterShape::Poster),
            poster: self.poster.from_protobuf(),
            background: self.background.from_protobuf(),
            logo: self.logo.from_protobuf(),
            description: self.description.to_owned(),
            release_info: self.release_info.to_owned(),
            runtime: self.runtime.to_owned(),
            released: self.released.from_protobuf(),
            links: Default::default(),
            trailer_streams: Default::default(),
            behavior_hints: self.behavior_hints.from_protobuf(),
        }
    }
}

impl ToProtobuf<types::MetaItemBehaviorHints, ()> for MetaItemBehaviorHints {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::MetaItemBehaviorHints {
        types::MetaItemBehaviorHints {
            default_video_id: self.default_video_id.clone(),
            featured_video_id: self.featured_video_id.clone(),
            has_scheduled_videos: self.has_scheduled_videos,
        }
    }
}

impl ToProtobuf<types::MetaItemDeepLinks, ()> for MetaItemDeepLinks {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> types::MetaItemDeepLinks {
        types::MetaItemDeepLinks {
            meta_details_videos: self.meta_details_videos.clone(),
            meta_details_streams: self.meta_details_streams.clone(),
            player: self.player.clone(),
        }
    }
}

impl ToProtobuf<types::MetaItemPreview, (&Ctx, &ResourceRequest)> for MetaItemPreview {
    fn to_protobuf<E: Env + 'static>(
        &self,
        (ctx, meta_request): &(&Ctx, &ResourceRequest),
    ) -> types::MetaItemPreview {
        types::MetaItemPreview {
            id: self.id.to_string(),
            r#type: self.r#type.to_string(),
            name: self.name.to_string(),
            poster_shape: self.poster_shape.to_protobuf::<E>(&()) as i32,
            poster: self.poster.to_protobuf::<E>(&()),
            background: self.background.to_protobuf::<E>(&()),
            logo: self.logo.to_protobuf::<E>(&()),
            description: self.description.clone(),
            release_info: self.release_info.clone(),
            runtime: self.runtime.clone(),
            released: self.released.to_protobuf::<E>(&()),
            links: self.links.to_protobuf::<E>(&()),
            behavior_hints: self.behavior_hints.to_protobuf::<E>(&()),
            deep_links: MetaItemDeepLinks::from((self, *meta_request)).to_protobuf::<E>(&()),
            in_library: ctx
                .library
                .items
                .get(&self.id)
                .map(|library_item| !library_item.removed)
                .unwrap_or_default(),
            watched: ctx
                .library
                .items
                .get(&self.id)
                .map(|library_item| library_item.watched())
                .unwrap_or_default(),
            in_cinema: self
                .released
                .filter(|_released| self.r#type == "movie")
                .map(|released| released + Duration::days(30) > E::now())
                .unwrap_or_default(),
        }
    }
}
