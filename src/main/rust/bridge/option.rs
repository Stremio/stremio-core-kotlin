use jni::objects::JObject;
use jni::JNIEnv;

use crate::bridge::{ToProtobuf, TryFromKotlin, TryIntoKotlin};

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

impl<T: ToProtobuf<U, A>, U, A> ToProtobuf<Option<U>, A> for Option<T> {
    fn to_protobuf(&self, args: &A) -> Option<U> {
        self.as_ref().map(|item| item.to_protobuf(args))
    }
}
