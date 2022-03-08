use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
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
