use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::addon::ExtraValue;

impl<'a> TryIntoKotlin<'a, ()> for ExtraValue {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let value = self.value.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::ExtraValue).unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[name.as_obj().into(), value.as_obj().into()],
        )
    }
}

impl TryFromKotlin for ExtraValue {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let name = env
            .call_method(value, "getName", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let name = String::try_from_kotlin(name.as_obj(), env)?;
        let value = env
            .call_method(value, "getValue", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let value = String::try_from_kotlin(value.as_obj(), env)?;
        Ok(ExtraValue { name, value })
    }
}
