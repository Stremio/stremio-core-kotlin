use stremio_core::models::continue_watching_preview::{ContinueWatchingPreview, Item};
use stremio_core::models::ctx::Ctx;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::{models, types};

impl ToProtobuf<types::LibraryItem, Ctx> for Item {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> types::LibraryItem {
        self.library_item
            .to_protobuf::<E>(&(ctx, Some(self.notifications)))
    }
}

impl ToProtobuf<models::ContinueWatchingPreview, Ctx> for ContinueWatchingPreview {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        ctx: &Ctx,
    ) -> models::ContinueWatchingPreview {
        models::ContinueWatchingPreview {
            library_items: self.items.to_protobuf::<E>(ctx),
        }
    }
}
