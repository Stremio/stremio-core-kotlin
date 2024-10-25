use stremio_core::types::resource::PosterShape;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<PosterShape> for types::PosterShape {
    fn from_protobuf(&self) -> PosterShape {
        match self {
            types::PosterShape::Poster => PosterShape::Poster,
            types::PosterShape::Landscape => PosterShape::Landscape,
            types::PosterShape::Square => PosterShape::Square,
        }
    }
}

impl ToProtobuf<types::PosterShape, ()> for PosterShape {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::PosterShape {
        match self {
            PosterShape::Poster => types::PosterShape::Poster,
            PosterShape::Landscape => types::PosterShape::Landscape,
            PosterShape::Square => types::PosterShape::Square,
        }
    }
}
