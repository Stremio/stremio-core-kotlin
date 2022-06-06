use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::addon::ExtraValue;

use crate::bridge::{ToProtobuf, TryFromKotlin};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::types;

impl TryFromKotlin for ExtraValue {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let name = env
            .call_method(value, "getName", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let name = String::try_from_kotlin(name.as_obj(), env)?;
        let value = env
            .call_method(value, "getValue", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let value = String::try_from_kotlin(value.as_obj(), env)?;
        Ok(ExtraValue { name, value })
    }
}

impl ToProtobuf<types::ExtraValue, ()> for ExtraValue {
    fn to_protobuf(&self, _args: &()) -> types::ExtraValue {
        types::ExtraValue {
            name: self.name.to_string(),
            value: self.value.to_string(),
        }
    }
}
