use jni::JNIEnv;
use jni::objects::JObject;
use stremio_core::types::resource::Link;
use url::Url;

use crate::bridge::{ToProtobuf, TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for Link {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let name = env
            .call_method(
                value,
                "getName",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let name = String::try_from_kotlin(name.as_obj(), env)?;
        let category = env
            .call_method(
                value,
                "getCategory",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let category = String::try_from_kotlin(category.as_obj(), env)?;
        let url = env
            .call_method(
                value,
                "getUrl",
                format!("()L{};", KotlinClassName::String.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let url = String::try_from_kotlin(url.as_obj(), env)?;
        let url = Url::parse(&url).expect("Link.url parse failed");
        Ok(Link {
            name,
            category,
            url,
        })
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Link {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let name = self.name.try_into_kotlin(&(), env)?.auto_local(env);
        let category = self.category.try_into_kotlin(&(), env)?.auto_local(env);
        let url = self.url.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::Link).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::String.value(),
                KotlinClassName::String.value(),
                KotlinClassName::String.value()
            ),
            &[
                name.as_obj().into(),
                category.as_obj().into(),
                url.as_obj().into(),
            ],
        )
    }
}

impl ToProtobuf<types::Link, ()> for Link {
    fn to_protobuf(&self, _args: &()) -> types::Link {
        types::Link {
            name: self.name.to_string(),
            category: self.category.to_string(),
            url: self.url.to_string(),
        }
    }
}
