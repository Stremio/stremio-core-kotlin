use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_deeplinks::ExternalPlayerLink;

impl<'a> TryIntoKotlin<'a, ()> for ExternalPlayerLink {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let href = self.href.try_into_kotlin(&(), env)?.auto_local(env);
        let download = self.download.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::ExternalPlayerLink).unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[href.as_obj().into(), download.as_obj().into()],
        )
    }
}
