use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::common::ResourceLoadable;

impl<'a, U, R: TryIntoKotlin<'a, U>> TryIntoKotlin<'a, (Option<String>, U)>
    for ResourceLoadable<R>
{
    #[inline]
    fn try_into_kotlin(
        &self,
        args: &(Option<String>, U),
        env: &JNIEnv<'a>,
    ) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let title = args.0.try_into_kotlin(&(), env)?.auto_local(env);
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let content = self.content.try_into_kotlin(&args.1, env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::ResourceLoadable).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::ResourceRequest.value(),
                KotlinClassName::Loadable.value()
            ),
            &[
                title.as_obj().into(),
                request.as_obj().into(),
                content.as_obj().into(),
            ],
        )
    }
}
