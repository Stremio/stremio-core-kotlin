use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::KotlinClassName;
use chrono::{DateTime, TimeZone, Utc};
use jni::objects::JObject;
use jni::JNIEnv;

impl<'a> TryIntoKotlin<'a, ()> for DateTime<Utc> {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        env.new_object(
            KotlinClassName::Date.value(),
            "(J)V",
            &[self.timestamp_millis().into()],
        )
    }
}

impl TryFromKotlin for DateTime<Utc> {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let time = env.call_method(value, "getTime", "()J", &[])?.j()?;
        let (secs, nsecs) = (time / 1000, time % 1000 * 1_000_000);
        Ok(Utc.timestamp(secs, nsecs as u32))
    }
}
