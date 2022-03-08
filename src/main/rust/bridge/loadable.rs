use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use std::fmt;
use stremio_core::models::common::Loadable;

impl<'a, T, R: TryIntoKotlin<'a, T>, E: fmt::Display> TryIntoKotlin<'a, T> for Loadable<R, E> {
    #[inline]
    fn try_into_kotlin(&self, args: &T, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            Loadable::Ready(ready) => {
                let ready = ready.try_into_kotlin(args, env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Loadable_Ready).unwrap(),
                    "(Ljava/lang/Object;)V",
                    &[ready.as_obj().into()],
                )
            }
            Loadable::Err(error) => {
                let message = error.to_string().try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Loadable_Error).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[message.as_obj().into()],
                )
            }
            Loadable::Loading => env.new_object(
                classes.get(&KotlinClassName::Loadable_Loading).unwrap(),
                "()V",
                &[],
            ),
        }
    }
}
