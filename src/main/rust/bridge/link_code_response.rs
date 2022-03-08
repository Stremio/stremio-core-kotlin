use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::api::LinkCodeResponse;

impl<'a> TryIntoKotlin<'a, ()> for LinkCodeResponse {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let code = self.code.try_into_kotlin(&(), env)?.auto_local(env);
        let link = self.link.try_into_kotlin(&(), env)?.auto_local(env);
        let qrcode = self.qrcode.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LinkCodeResponse).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[
                code.as_obj().into(),
                link.as_obj().into(),
                qrcode.as_obj().into(),
            ],
        )
    }
}
