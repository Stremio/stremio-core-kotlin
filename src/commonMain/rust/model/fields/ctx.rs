use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;
use stremio_core::models::ctx::Ctx;

impl ToProtobuf<models::Ctx, ()> for Ctx {
    fn to_protobuf(&self, _args: &()) -> models::Ctx {
        models::Ctx {
            profile: self.profile.to_protobuf(&()),
            events: self.events.to_protobuf(&()),
        }
    }
}
