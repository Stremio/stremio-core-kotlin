use stremio_core::models::link::Link;
use stremio_core::types::api::LinkAuthKey;

use crate::bridge::{ToProtobuf, ToProtobufAny};
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::AuthLink, ()> for Link<LinkAuthKey> {
    fn to_protobuf(&self, _args: &()) -> models::AuthLink {
        models::AuthLink {
            code: self.code.to_protobuf(&()),
            data: self.data.to_protobuf(&()),
        }
    }
}
