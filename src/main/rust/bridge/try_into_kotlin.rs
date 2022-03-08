use auto_impl::auto_impl;
use jni::objects::JObject;
use jni::JNIEnv;

#[auto_impl(&)]
pub trait TryIntoKotlin<'a, T> {
    fn try_into_kotlin(&self, args: &T, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>>;
}
