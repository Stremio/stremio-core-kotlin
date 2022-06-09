use crate::bridge::FromProtobuf;
use crate::model::AndroidModelField;
use crate::protobuf::stremio::core::runtime::Field;

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
            Field::StreamingServer => AndroidModelField::StreamingServer,
        }
    }
}
