use stremio_core::models::ctx::Ctx;
use stremio_core::models::player::{Player, Selected};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Selected> for models::player::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            stream: self.stream.from_protobuf(),
            stream_request: self.stream_request.from_protobuf(),
            meta_request: self.meta_request.from_protobuf(),
            subtitles_path: self.subtitles_path.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::player::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::player::Selected {
        models::player::Selected {
            stream: self.stream.to_protobuf(&(None, None, None)),
            stream_request: self.stream_request.to_protobuf(&()),
            meta_request: self.meta_request.to_protobuf(&()),
            subtitles_path: self.subtitles_path.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::Player, Ctx> for Player {
    fn to_protobuf(&self, ctx: &Ctx) -> models::Player {
        models::Player {
            selected: self.selected.to_protobuf(&()),
            meta_item: self.meta_item.as_ref().to_protobuf(&(
                ctx,
                self.library_item.as_ref(),
                None,
            )),
            subtitles: self.subtitles.to_protobuf(ctx),
            next_video: self
                .next_video
                .to_protobuf(&(self.library_item.as_ref(), None, None)),
            series_info: self.series_info.to_protobuf(&()),
            library_item: self.library_item.to_protobuf(&()),
        }
    }
}
