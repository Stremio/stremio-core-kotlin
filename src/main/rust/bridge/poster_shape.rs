use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::types::resource::PosterShape;

use crate::bridge::{ToProtobufAny, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl<'a> TryIntoKotlin<'a, ()> for PosterShape {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        env.get_static_field(
            classes.get(&KotlinClassName::PosterShape).unwrap(),
            match self {
                PosterShape::Poster => "Poster",
                PosterShape::Landscape => "Landscape",
                PosterShape::Square => "Square",
            },
            format!("L{};", KotlinClassName::PosterShape.value()),
        )?
        .l()
    }
}

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
