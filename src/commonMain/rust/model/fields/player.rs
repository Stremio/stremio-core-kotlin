use stremio_core::models::ctx::Ctx;
use stremio_core::models::player::{Player, Selected, VideoParams};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Selected> for models::player::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            stream: self.stream.from_protobuf(),
            stream_request: self.stream_request.from_protobuf(),
            meta_request: self.meta_request.from_protobuf(),
            subtitles_path: self.subtitles_path.from_protobuf(),
            video_params: self.video_params.from_protobuf(),
        }
    }
}

impl FromProtobuf<VideoParams> for models::player::VideoParams {
    fn from_protobuf(&self) -> VideoParams {
        VideoParams {
            hash: self.hash.to_owned(),
            size: self.size.map(|x| x as u64).to_owned(),
        }
    }
}

impl ToProtobuf<models::player::VideoParams, ()> for VideoParams {
    fn to_protobuf(&self, _args: &()) -> models::player::VideoParams {
        models::player::VideoParams {
            hash: self.hash.to_owned(),
            size: self.size.map(|x| x as i64).to_owned(),
        }
    }
}

impl ToProtobuf<models::player::Selected, Ctx> for Selected {
    fn to_protobuf(&self, ctx: &Ctx) -> models::player::Selected {
        let addon_name = self.stream_request.as_ref().and_then(|request| {
            ctx.profile
                .addons
                .iter()
                .find(|addon| addon.transport_url == request.base)
                .map(|addon| &addon.manifest.name)
        });

        models::player::Selected {
            stream: self
                .stream
                .to_protobuf(&(Some(ctx), addon_name, None, None)),
            stream_request: self.stream_request.to_protobuf(&()),
            meta_request: self.meta_request.to_protobuf(&()),
            subtitles_path: self.subtitles_path.to_protobuf(&()),
            video_params: self.video_params.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::Player, Ctx> for Player {
    fn to_protobuf(&self, ctx: &Ctx) -> models::Player {
        models::Player {
            selected: self.selected.to_protobuf(ctx),
            meta_item: self.meta_item.as_ref().to_protobuf(&(
                ctx,
                self.library_item.as_ref(),
                self.watched.as_ref(),
            )),
            subtitles: self.subtitles.to_protobuf(ctx),
            next_video: self.next_video.to_protobuf(&(
                self.library_item.as_ref(),
                self.watched.as_ref(),
                None,
            )),
            series_info: self.series_info.to_protobuf(&()),
            library_item: self.library_item.to_protobuf(&()),
        }
    }
}
