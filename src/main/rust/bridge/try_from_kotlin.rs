use jni::objects::JObject;
use jni::JNIEnv;

pub trait TryFromKotlin {
    fn try_from_kotlin<'a>(obj: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self>
    where
        Self: Sized;
}
