use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::addon::{ResourcePath, ResourceRequest};
use url::Url;

use crate::bridge::{ToProtobuf, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for ResourceRequest {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let base = env
            .call_method(value, "getBase", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let base = String::try_from_kotlin(base.as_obj(), env)?;
        let base = Url::parse(&base).expect("ResourceRequest.base parse failed");
        let path = env
            .call_method(
                value,
                "getPath",
                format!("()L{};", KotlinClassName::ResourcePath.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let path = ResourcePath::try_from_kotlin(path.as_obj(), env)?;
        Ok(ResourceRequest { base, path })
    }
}

impl ToProtobuf<types::ResourceRequest, ()> for ResourceRequest {
    fn to_protobuf(&self, _args: &()) -> types::ResourceRequest {
        types::ResourceRequest {
            base: self.base.to_string(),
            path: self.path.to_protobuf(&()),
        }
    }
}
