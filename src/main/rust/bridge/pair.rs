use crate::bridge::{ToProtobuf, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use crate::protobuf::stremio::core::runtime;

impl<'a, T: TryIntoKotlin<'a, V>, U: TryIntoKotlin<'a, V>, V> TryIntoKotlin<'a, V> for (T, U) {
    #[inline]
    fn try_into_kotlin(&self, args: &V, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let first = self.0.try_into_kotlin(args, env)?.auto_local(env);
        let second = self.1.try_into_kotlin(args, env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Pair).unwrap(),
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            &[first.as_obj().into(), second.as_obj().into()],
        )
    }
}

impl ToProtobuf<runtime::PlanPair, ()> for (Vec<String>, Vec<String>) {
    fn to_protobuf(&self, _args: &()) -> runtime::PlanPair {
        runtime::PlanPair {
            first: self.0.clone(),
            second: self.1.clone()
        }
    }
}
