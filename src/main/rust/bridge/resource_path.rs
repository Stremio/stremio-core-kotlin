use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::addon::{ExtraValue, ResourcePath};

use crate::bridge::{ToProtobuf, TryFromKotlin};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for ResourcePath {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let resource = env
            .call_method(value, "getResource", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let resource = String::try_from_kotlin(resource.as_obj(), env)?;
        let r#type = env
            .call_method(value, "getType", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let r#type = String::try_from_kotlin(r#type.as_obj(), env)?;
        let id = env
            .call_method(value, "getId", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let id = String::try_from_kotlin(id.as_obj(), env)?;
        let extra = env
            .call_method(value, "getExtra", "()Ljava/util/List;", &[])?
            .l()?
            .auto_local(env);
        let extra = Vec::<ExtraValue>::try_from_kotlin(extra.as_obj(), env)?;
        Ok(ResourcePath {
            resource,
            r#type,
            id,
            extra,
        })
    }
}

impl ToProtobuf<types::ResourcePath, ()> for ResourcePath {
    fn to_protobuf(&self, _args: &()) -> types::ResourcePath {
        types::ResourcePath {
            resource: self.resource.to_string(),
            r#type: self.r#type.to_string(),
            id: self.id.to_string(),
            extra: self.extra.to_protobuf(&()),
        }
    }
}
