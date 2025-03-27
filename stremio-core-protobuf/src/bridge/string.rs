use crate::bridge::{FromProtobuf, ToProtobuf};
use stremio_core::types::profile::{Password, UserId};
use url::Url;

impl FromProtobuf<Url> for String {
    fn from_protobuf(&self) -> Url {
        Url::parse(self).expect("url parse failed")
    }
}

impl FromProtobuf<Password> for String {
    fn from_protobuf(&self) -> Password {
        Password(self.to_owned())
    }
}
impl FromProtobuf<UserId> for String {
    fn from_protobuf(&self) -> UserId {
        UserId(self.to_owned())
    }
}

impl ToProtobuf<String, ()> for Url {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> String {
        self.to_string()
    }
}
