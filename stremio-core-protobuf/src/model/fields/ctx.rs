use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;
use stremio_core::models::ctx::Ctx;

impl ToProtobuf<models::Ctx, ()> for Ctx {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> models::Ctx {
        models::Ctx {
            profile: self.profile.to_protobuf::<E>(&()),
            events: self.events.to_protobuf::<E>(&()),
            streaming_urls: self.streaming_server_urls.to_protobuf::<E>(&()),
        }
    }
}
