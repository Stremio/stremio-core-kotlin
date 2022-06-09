use stremio_core::types::resource::Subtitles;
use url::Url;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<Subtitles> for types::Subtitle {
    fn from_protobuf(&self) -> Subtitles {
        Subtitles {
            lang: self.lang.to_string(),
            url: Url::parse(self.url.as_str()).expect("Subtitle.url parse failed"),
        }
    }
}

impl ToProtobuf<types::Subtitle, ()> for Subtitles {
    fn to_protobuf(&self, _args: &()) -> types::Subtitle {
        types::Subtitle {
            lang: self.lang.to_string(),
            url: self.url.to_string(),
        }
    }
}
