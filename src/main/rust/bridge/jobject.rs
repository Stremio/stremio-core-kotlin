use crate::bridge::TryIntoKotlin;
use jni::objects::JObject;
use jni::JNIEnv;

impl<'a> TryIntoKotlin<'a, ()> for JObject<'a> {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), _env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        Ok(*self)
    }
}
