use stremio_core::models::link::Link;
use stremio_core::types::api::LinkAuthKey;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::AuthLink, ()> for Link<LinkAuthKey> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> models::AuthLink {
        models::AuthLink {
            code: self.code.to_protobuf::<E>(&()),
            data: self.data.to_protobuf::<E>(&()),
        }
    }
}
