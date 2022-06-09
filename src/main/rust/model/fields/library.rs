use std::cmp;
use std::convert::TryFrom;
use std::num::NonZeroUsize;

use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::library_with_filters::{
    LibraryRequest, LibraryRequestPage, LibraryWithFilters, Selectable, SelectablePage,
    SelectableSort, SelectableType, Selected, Sort,
};

use crate::bridge::{ToProtobuf, TryFromKotlin};
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use crate::model::LibraryByType;
use crate::protobuf::stremio::core::models;

impl TryFromKotlin for Sort {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let value = env
            .call_method(value, "getName", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let value = String::try_from_kotlin(value.as_obj(), env)?;
        match value.as_ref() {
            "LastWatched" => Ok(Sort::LastWatched),
            "Name" => Ok(Sort::Name),
            "TimesWatched" => Ok(Sort::TimesWatched),
            value => panic!("Invalid sort: {}", value),
        }
    }
}

impl TryFromKotlin for LibraryRequest {
    fn try_from_kotlin<'a>(request: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let r#type = env
            .call_method(request, "getType", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let r#type = Option::<String>::try_from_kotlin(r#type.as_obj(), env)?;
        let sort = env
            .call_method(
                request,
                "getSort",
                format!("()L{};", KotlinClassName::LibraryWithFilters_Sort.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let sort = Sort::try_from_kotlin(sort.as_obj(), env)?;
        let page = env.call_method(request, "getPage", "()J", &[])?.j()?;
        let page = usize::try_from(cmp::max(page, 1)).unwrap_or(usize::MAX);
        let page = LibraryRequestPage(NonZeroUsize::new(page).unwrap());
        Ok(LibraryRequest { r#type, sort, page })
    }
}

impl TryFromKotlin for Selected {
    fn try_from_kotlin<'a>(selected: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let request = env
            .call_method(
                selected,
                "getRequest",
                format!(
                    "()L{};",
                    KotlinClassName::LibraryWithFilters_LibraryRequest.value()
                ),
                &[],
            )?
            .l()?
            .auto_local(env);
        let request = LibraryRequest::try_from_kotlin(request.as_obj(), env)?;
        Ok(Selected { request })
    }
}

impl ToProtobuf<models::library_with_filters::Sort, ()> for Sort {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Sort {
        match self {
            Sort::LastWatched => models::library_with_filters::Sort::LastWatched,
            Sort::Name => models::library_with_filters::Sort::Name,
            Sort::TimesWatched => models::library_with_filters::Sort::TimesWatched,
        }
    }
}

impl ToProtobuf<models::library_with_filters::LibraryRequest, ()> for LibraryRequest {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::LibraryRequest {
        models::library_with_filters::LibraryRequest {
            r#type: self.r#type.clone(),
            sort: self.sort.to_protobuf(&()) as i32,
            page: i64::try_from(self.page.0.get()).unwrap_or(i64::MAX),
        }
    }
}

impl ToProtobuf<models::library_with_filters::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Selected {
        models::library_with_filters::Selected {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectableType, ()> for SelectableType {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectableType {
        models::library_with_filters::SelectableType {
            r#type: self.r#type.clone(),
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectableSort, ()> for SelectableSort {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectableSort {
        models::library_with_filters::SelectableSort {
            sort: self.sort.to_protobuf(&()) as i32,
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectablePage, ()> for SelectablePage {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectablePage {
        models::library_with_filters::SelectablePage {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::Selectable, ()> for Selectable {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Selectable {
        models::library_with_filters::Selectable {
            types: self.types.to_protobuf(&()),
            sorts: self.sorts.to_protobuf(&()),
            prev_page: self.prev_page.to_protobuf(&()),
            next_page: self.next_page.to_protobuf(&()),
        }
    }
}

impl<F> ToProtobuf<models::LibraryWithFilters, ()> for LibraryWithFilters<F> {
    fn to_protobuf(&self, _args: &()) -> models::LibraryWithFilters {
        models::LibraryWithFilters {
            selected: self.selected.to_protobuf(&()),
            selectable: self.selectable.to_protobuf(&()),
            catalog: self.catalog.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::LibraryByType, ()> for LibraryByType {
    fn to_protobuf(&self, _args: &()) -> models::LibraryByType {
        models::LibraryByType {
            catalogs: self.catalogs.to_protobuf(&()),
        }
    }
}
