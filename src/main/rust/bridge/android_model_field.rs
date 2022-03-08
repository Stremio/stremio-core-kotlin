use crate::bridge::TryFromKotlin;
use crate::jni_ext::JObjectExt;
use crate::model::AndroidModelField;
use jni::objects::JObject;
use jni::JNIEnv;

impl TryFromKotlin for AndroidModelField {
    fn try_from_kotlin<'a>(field: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let field = env
            .call_method(field, "getValue", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let field = String::try_from_kotlin(field.as_obj(), env)?;
        match field.as_str() {
            "ctx" => Ok(AndroidModelField::Ctx),
            "auth_link" => Ok(AndroidModelField::AuthLink),
            "continue_watching_preview" => Ok(AndroidModelField::ContinueWatchingPreview),
            "discover" => Ok(AndroidModelField::Discover),
            "library" => Ok(AndroidModelField::Library),
            "board" => Ok(AndroidModelField::Board),
            "search" => Ok(AndroidModelField::Search),
            "meta_details" => Ok(AndroidModelField::MetaDetails),
            "streaming_server" => Ok(AndroidModelField::StreamingServer),
            _ => unimplemented!("AndroidModelField: {}", field),
        }
    }
}
