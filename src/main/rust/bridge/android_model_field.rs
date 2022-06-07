use jni::objects::JObject;
use jni::JNIEnv;

use crate::bridge::{FromProtobuf, TryFromKotlin};
use crate::model::AndroidModelField;
use crate::protobuf::stremio::core::runtime::Field;

impl TryFromKotlin for AndroidModelField {
    fn try_from_kotlin<'a>(field: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let field = env.call_method(field, "getValue", "()I", &[])?.i()?;
        let field = Field::from_i32(field)
            .from_protobuf()
            .expect("AndroidModelField convert failed");
        Ok(field)
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
            Field::StreamingServer => AndroidModelField::StreamingServer,
        }
    }
}
