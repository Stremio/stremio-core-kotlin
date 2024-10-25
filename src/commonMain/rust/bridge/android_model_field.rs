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
            Field::DataExport => AndroidModelField::DataExport,
            Field::ContinueWatchingPreview => AndroidModelField::ContinueWatchingPreview,
            Field::Board => AndroidModelField::Board,
            Field::Discover => AndroidModelField::Discover,
            Field::Library => AndroidModelField::Library,
            Field::LibraryByType => AndroidModelField::LibraryByType,
            Field::ContinueWatching => AndroidModelField::ContinueWatching,
            Field::Search => AndroidModelField::Search,
            Field::LocalSearch => AndroidModelField::LocalSearch,
            Field::Addons => AndroidModelField::Addons,
            Field::MetaDetails => AndroidModelField::MetaDetails,
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
            AndroidModelField::DataExport => Field::DataExport,
            AndroidModelField::ContinueWatchingPreview => Field::ContinueWatchingPreview,
            AndroidModelField::Board => Field::Board,
            AndroidModelField::Discover => Field::Discover,
            AndroidModelField::Library => Field::Library,
            AndroidModelField::LibraryByType => Field::LibraryByType,
            AndroidModelField::ContinueWatching => Field::ContinueWatching,
            AndroidModelField::Search => Field::Search,
            AndroidModelField::LocalSearch => Field::LocalSearch,
            AndroidModelField::Addons => Field::Addons,
            AndroidModelField::MetaDetails => Field::MetaDetails,
            AndroidModelField::AddonDetails => Field::AddonDetails,
            AndroidModelField::StreamingServer => Field::StreamingServer,
            AndroidModelField::Player => Field::Player,
        }
    }
}
