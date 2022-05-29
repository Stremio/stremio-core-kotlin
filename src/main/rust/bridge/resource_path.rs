use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::types::addon::{ExtraValue, ResourcePath};

use crate::bridge::{ToProtobuf, ToProtobufAny, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl<'a> TryIntoKotlin<'a, ()> for ResourcePath {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let resource = self.resource.try_into_kotlin(&(), env)?.auto_local(env);
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let id = self.id.try_into_kotlin(&(), env)?.auto_local(env);
        let extra = self.extra.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::ResourcePath).unwrap(),
            format!(
                "(L{};L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                "java/util/List"
            ),
            &[
                resource.as_obj().into(),
                r#type.as_obj().into(),
                id.as_obj().into(),
                extra.as_obj().into(),
            ],
        )
    }
}

impl TryFromKotlin for ResourcePath {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let resource = env
            .call_method(value, "getResource", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let resource = String::try_from_kotlin(resource.as_obj(), env)?;
        let r#type = env
            .call_method(value, "getType", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let r#type = String::try_from_kotlin(r#type.as_obj(), env)?;
        let id = env
            .call_method(value, "getId", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let id = String::try_from_kotlin(id.as_obj(), env)?;
        let extra = env
            .call_method(value, "getExtra", "()Ljava/util/List;", &[])?
            .l()?
            .auto_local(env);
        let extra = Vec::<ExtraValue>::try_from_kotlin(extra.as_obj(), env)?;
        Ok(ResourcePath {
            resource,
            r#type,
            id,
            extra,
        })
    }
}

impl ToProtobuf<types::ResourcePath, ()> for ResourcePath {
    fn to_protobuf(&self, _args: &()) -> types::ResourcePath {
        types::ResourcePath {
            resource: self.resource.to_string(),
            r#type: self.r#type.to_string(),
            id: self.id.to_string(),
            extra: self.extra.to_protobuf(&()),
        }
    }
}
