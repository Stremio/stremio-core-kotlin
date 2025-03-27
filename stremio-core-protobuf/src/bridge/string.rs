use crate::bridge::{FromProtobuf, ToProtobuf};
use stremio_core::types::profile::{Password, UserId};
use url::Url;

use super::TryFromProtobuf;

impl FromProtobuf<Url> for String {
    fn from_protobuf(&self) -> Url {
        Url::parse(self).expect("url parse failed")
    }
}

impl TryFromProtobuf<Url> for String {
    type Error = url::ParseError;

    fn try_from_protobuf(&self) -> Result<Url, Self::Error> {
        Url::parse(self)
    }
}
impl FromProtobuf<Url> for &str {
    fn from_protobuf(&self) -> Url {
        Url::parse(self).expect("url parse failed")
    }
}

impl TryFromProtobuf<Url> for &str {
    type Error = url::ParseError;

    fn try_from_protobuf(&self) -> Result<Url, Self::Error> {
        Url::parse(self)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_parsing_url_from_protobuf() {
        let _url: Url = "/bad-url".from_protobuf();
    }
    #[test]
    #[should_panic]
    fn test_parsing_url_from_protobuf_1() {
        let _url: Url = "#".from_protobuf();
    }
    #[test]
    fn test_parsing_url_try_from_protobuf() {
        let url_result = "#".try_from_protobuf();

        assert!(url_result.is_err());
    }
}
