use std::convert::TryFrom;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Hash, EnumIter)]
#[allow(non_camel_case_types)]
pub enum KotlinClassName {
    Map,
    HashMap,
    Core,
    Storage_Result,
    Storage_Result_Ok,
    Storage_Result_Err,
    EnvError,
}

impl TryFrom<String> for KotlinClassName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        KotlinClassName::iter()
            .find(|class_name| class_name.value() == value)
            .ok_or(format!("Class name not found: {}", value))
    }
}

impl KotlinClassName {
    pub fn value(&self) -> &str {
        match self {
            KotlinClassName::Map => "java/util/Map",
            KotlinClassName::HashMap => "java/util/HashMap",
            KotlinClassName::Core => "com/stremio/core/Core",
            KotlinClassName::Storage_Result => "com/stremio/core/Storage$Result",
            KotlinClassName::Storage_Result_Ok => "com/stremio/core/Storage$Result$Ok",
            KotlinClassName::Storage_Result_Err => "com/stremio/core/Storage$Result$Err",
            KotlinClassName::EnvError => "com/stremio/core/runtime/EnvError",
        }
    }
}
