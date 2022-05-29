use crate::bridge::{ToProtobuf, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::ctx::Ctx;

impl<'a> TryIntoKotlin<'a, ()> for Ctx {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let profile = self.profile.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Ctx).unwrap(),
            format!("(L{};)V", KotlinClassName::Profile.value()),
            &[profile.as_obj().into()],
        )
    }
}

impl ToProtobuf<models::Ctx, ()> for Ctx {
    fn to_protobuf(&self, _args: &()) -> models::Ctx {
        models::Ctx {
            profile: self.profile.to_protobuf(&()),
        }
    }
}
