use stremio_core::runtime::Env;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::env::AndroidEnv;
use crate::model::AndroidModelField;
use crate::protobuf::stremio::core::runtime::Field;

impl From<Field> for AndroidModelField {
    fn from(android_field: Field) -> Self {
        android_field.from_protobuf()
    }
}

impl From<AndroidModelField> for Field {
    fn from(field: AndroidModelField) -> Self {
        field.to_protobuf::<AndroidEnv>(&())
    }
}

impl FromProtobuf<AndroidModelField> for Field {
    fn from_protobuf(&self) -> AndroidModelField {
        match self {
            Field::Ctx => AndroidModelField::Ctx,
            Field::AuthLink => AndroidModelField::AuthLink,
            Field::ContinueWatchingPreview => AndroidModelField::ContinueWatchingPreview,
            Field::Discover => AndroidModelField::Discover,
            Field::Library => AndroidModelField::Library,
            Field::LibraryByType => AndroidModelField::LibraryByType,
            Field::Board => AndroidModelField::Board,
            Field::Search => AndroidModelField::Search,
            Field::MetaDetails => AndroidModelField::MetaDetails,
            Field::Addons => AndroidModelField::Addons,
            Field::AddonDetails => AndroidModelField::AddonDetails,
            Field::StreamingServer => AndroidModelField::StreamingServer,
            Field::Player => AndroidModelField::Player,
        }
    }
}

impl ToProtobuf<Field, ()> for AndroidModelField {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> Field {
        match self {
            AndroidModelField::Ctx => Field::Ctx,
            AndroidModelField::AuthLink => Field::AuthLink,
            AndroidModelField::ContinueWatchingPreview => Field::ContinueWatchingPreview,
            AndroidModelField::Discover => Field::Discover,
            AndroidModelField::Library => Field::Library,
            AndroidModelField::LibraryByType => Field::LibraryByType,
            AndroidModelField::Board => Field::Board,
            AndroidModelField::Search => Field::Search,
            AndroidModelField::MetaDetails => Field::MetaDetails,
            AndroidModelField::Addons => Field::Addons,
            AndroidModelField::AddonDetails => Field::AddonDetails,
            AndroidModelField::StreamingServer => Field::StreamingServer,
            AndroidModelField::Player => Field::Player,
        }
    }
}
