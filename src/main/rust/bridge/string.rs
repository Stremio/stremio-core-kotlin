use crate::bridge::{FromProtobuf, ToProtobuf};
use url::Url;

impl FromProtobuf<Url> for String {
    fn from_protobuf(&self) -> Url {
        Url::parse(&self).expect("url parse failed")
    }
}

impl ToProtobuf<String, ()> for Url {
    fn to_protobuf(&self, _args: &()) -> String {
        self.to_string()
    }
}
