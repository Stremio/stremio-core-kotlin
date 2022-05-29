use crate::bridge::{ToProtobuf, ToProtobufAny, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::continue_watching_preview::ContinueWatchingPreview;
use crate::protobuf::stremio::core::models;

impl<'a> TryIntoKotlin<'a, ()> for ContinueWatchingPreview {
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let library_items = self
            .library_items
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::ContinueWatchingPreview)
                .unwrap(),
            "(Ljava/util/List;)V",
            &[library_items.as_obj().into()],
        )
    }
}

impl ToProtobuf<models::ContinueWatchingPreview, ()> for ContinueWatchingPreview {
    fn to_protobuf(&self, _args: &()) -> models::ContinueWatchingPreview {
        models::ContinueWatchingPreview {
            library_items: self.library_items.to_protobuf(&())
        }
    }
}
