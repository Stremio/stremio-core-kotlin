use crate::bridge::{ToProtobuf, TryIntoKotlin};
use jni::objects::JObject;
use jni::JNIEnv;
use url::Url;

impl<'a> TryIntoKotlin<'a, ()> for String {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        env.new_string(&self).map(|string| string.into())
    }
}

impl ToProtobuf<String, ()> for Url {
    fn to_protobuf(&self, _args: &()) -> String {
        self.to_string()
    }
}
