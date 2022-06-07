use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::catalogs_with_extra::{CatalogsWithExtra, Selected};
use stremio_core::models::ctx::Ctx;
use stremio_core::types::addon::ExtraValue;

use crate::bridge::{FromProtobuf, ToProtobuf, TryFromKotlin};
use crate::jni_ext::JObjectExt;
use crate::protobuf::stremio::core::models;

impl TryFromKotlin for Selected {
    fn try_from_kotlin<'a>(selected: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let r#type = env
            .call_method(selected, "getType", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let r#type = Option::<String>::try_from_kotlin(r#type.as_obj(), env)?;
        let extra = env
            .call_method(selected, "getExtra", "()Ljava/util/List;", &[])?
            .l()?
            .auto_local(env);
        let extra = Vec::<ExtraValue>::try_from_kotlin(extra.as_obj(), env)?;
        Ok(Selected { r#type, extra })
    }
}

impl FromProtobuf<Selected> for models::catalogs_with_extra::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            r#type: self.r#type.clone(),
            extra: self.extra.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::catalogs_with_extra::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::catalogs_with_extra::Selected {
        models::catalogs_with_extra::Selected {
            r#type: self.r#type.clone(),
            extra: self.extra.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::CatalogsWithExtra, Ctx> for CatalogsWithExtra {
    fn to_protobuf(&self, ctx: &Ctx) -> models::CatalogsWithExtra {
        models::CatalogsWithExtra {
            selected: self.selected.to_protobuf(&()),
            catalogs: self.catalogs.to_protobuf(ctx),
        }
    }
}
