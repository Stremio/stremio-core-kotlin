use chrono::{DateTime, TimeZone, Utc};
use jni::objects::JObject;
use jni::JNIEnv;

use crate::bridge::{FromProtobuf, ToProtobuf, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::protobuf::google::protobuf::Timestamp;

impl<'a> TryIntoKotlin<'a, ()> for DateTime<Utc> {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        env.new_object(
            classes.get(&KotlinClassName::Date).unwrap(),
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

impl FromProtobuf<DateTime<Utc>> for Timestamp {
    fn from_protobuf(&self) -> DateTime<Utc> {
        Utc.timestamp(self.seconds, self.nanos as u32)
    }
}

impl ToProtobuf<Timestamp, ()> for DateTime<Utc> {
    fn to_protobuf(&self, _args: &()) -> Timestamp {
        Timestamp {
            seconds: self.timestamp(),
            nanos: self.timestamp_subsec_nanos() as i32,
        }
    }
}
