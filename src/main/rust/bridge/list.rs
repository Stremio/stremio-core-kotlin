use jni::objects::JObject;
use jni::JNIEnv;
use prost::Message;

use crate::bridge::{FromProtobuf, ToProtobuf, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;

impl<'a, T: TryIntoKotlin<'a, U>, U> TryIntoKotlin<'a, U> for Vec<T> {
    #[inline]
    fn try_into_kotlin(&self, args: &U, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let list = env.new_object(
            classes.get(&KotlinClassName::ArrayList).unwrap(),
            "()V",
            &[],
        )?;
        let list = env.get_list(list)?;
        for item in self.iter() {
            let item = item.try_into_kotlin(args, env)?.auto_local(env);
            list.add(item.as_obj())?;
        }
        Ok(list.into())
    }
}

impl<T: TryFromKotlin> TryFromKotlin for Vec<T> {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        env.get_list(value)?
            .iter()?
            .map(|extra_value| T::try_from_kotlin(extra_value, env))
            .collect::<jni::errors::Result<Vec<_>>>()
    }
}

impl<T: FromProtobuf<U>, U> FromProtobuf<Vec<U>> for Vec<T> {
    fn from_protobuf(&self) -> Vec<U> {
        self.iter().map(|item| item.from_protobuf()).collect()
    }
}

impl<T: ToProtobuf<U, A>, U: Message, A> ToProtobuf<Vec<U>, A> for Vec<T> {
    fn to_protobuf(&self, args: &A) -> Vec<U> {
        self.iter().map(|item| item.to_protobuf(args)).collect()
    }
}
