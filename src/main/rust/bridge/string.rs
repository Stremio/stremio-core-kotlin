use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use jni::objects::JObject;
use jni::JNIEnv;
use url::Url;

impl<'a> TryIntoKotlin<'a, ()> for String {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        env.new_string(&self).map(|string| string.into())
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Url {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        env.new_string(&self).map(|string| string.into())
    }
}

impl TryFromKotlin for String {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        Ok(env.get_string(value.into())?.to_string_lossy().into_owned())
    }
}
