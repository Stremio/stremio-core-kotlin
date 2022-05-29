use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::resource::PosterShape;

use crate::bridge::{ToProtobufAny, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for PosterShape {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let name = env
            .call_method(
                value,
                "name",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let name = String::try_from_kotlin(name.as_obj(), env)?;
        match name.as_ref() {
            "Poster" => Ok(PosterShape::Poster),
            "Landscape" => Ok(PosterShape::Landscape),
            _ => Ok(PosterShape::Square),
        }
    }
}

impl ToProtobufAny<types::PosterShape, ()> for PosterShape {
    fn to_protobuf(&self, _args: &()) -> types::PosterShape {
        match self {
            PosterShape::Poster => types::PosterShape::Poster,
            PosterShape::Landscape => types::PosterShape::Landscape,
            PosterShape::Square => types::PosterShape::Square,
        }
    }
}
