use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use jni::objects::JObject;
use jni::JNIEnv;

impl<'a, T: TryIntoKotlin<'a, U>, U> TryIntoKotlin<'a, U> for Option<T> {
    #[inline]
    fn try_into_kotlin(&self, args: &U, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        match &self {
            Some(value) => value.try_into_kotlin(args, env),
            None => Ok(JObject::null()),
        }
    }
}

impl<T: TryFromKotlin> TryFromKotlin for Option<T> {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        if value.is_null() {
            return Ok(None);
        }

        Ok(Some(T::try_from_kotlin(value, env)?))
    }
}
