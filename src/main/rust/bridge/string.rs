use crate::bridge::ToProtobuf;
use url::Url;

impl ToProtobuf<String, ()> for Url {
    fn to_protobuf(&self, _args: &()) -> String {
        self.to_string()
    }
}
