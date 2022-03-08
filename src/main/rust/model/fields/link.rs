use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::link::Link;
use stremio_core::types::api::LinkAuthKey;

impl<'a> TryIntoKotlin<'a, ()> for LinkAuthKey {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let auth_key = self.auth_key.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LinkAuthKey).unwrap(),
            format!("(L{};)V", KotlinClassName::String.value()),
            &[auth_key.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Link<LinkAuthKey> {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let code = self.code.try_into_kotlin(&(), env)?.auto_local(env);
        let data = self.data.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LinkModel).unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::Loadable.value(),
                KotlinClassName::Loadable.value()
            ),
            &[code.as_obj().into(), data.as_obj().into()],
        )
    }
}
