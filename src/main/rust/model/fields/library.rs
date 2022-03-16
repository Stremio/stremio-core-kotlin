use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use std::cmp;
use std::convert::TryFrom;
use std::num::NonZeroUsize;
use stremio_core::models::library_with_filters::{
    LibraryRequest, LibraryRequestPage, LibraryWithFilters, Selectable, SelectablePage,
    SelectableSort, SelectableType, Selected, Sort,
};
use stremio_deeplinks::LibraryDeepLinks;

impl TryFromKotlin for Sort {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let value = env
            .call_method(value, "getValue", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let value = String::try_from_kotlin(value.as_obj(), env)?;
        match value.as_ref() {
            "lastwatched" => Ok(Sort::LastWatched),
            "name" => Ok(Sort::Name),
            "timeswatched" => Ok(Sort::TimesWatched),
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

impl<'a> TryIntoKotlin<'a, ()> for LibraryDeepLinks {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let library = self.library.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LibraryDeepLinks).unwrap(),
            format!("(L{};)V", KotlinClassName::String.value()),
            &[library.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Sort {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        env.get_static_field(
            classes
                .get(&KotlinClassName::LibraryWithFilters_Sort)
                .unwrap(),
            match self {
                Sort::LastWatched => "LastWatched",
                Sort::Name => "Name",
                Sort::TimesWatched => "TimesWatched",
            },
            format!("L{};", KotlinClassName::LibraryWithFilters_Sort.value()),
        )?
        .l()
    }
}

impl<'a> TryIntoKotlin<'a, ()> for LibraryRequest {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let sort = self.sort.try_into_kotlin(&(), env)?.auto_local(env);
        let page = i64::try_from(self.page.0.get()).unwrap_or(i64::MAX).into();
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_LibraryRequest)
                .unwrap(),
            format!(
                "(Ljava/lang/String;L{};J)V",
                KotlinClassName::LibraryWithFilters_Sort.value()
            ),
            &[r#type.as_obj().into(), sort.as_obj().into(), page],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for Selected {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_Selected)
                .unwrap(),
            format!(
                "(L{};)V",
                KotlinClassName::LibraryWithFilters_LibraryRequest.value()
            ),
            &[request.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, String> for SelectableType {
    #[inline]
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let r#type = self.r#type.try_into_kotlin(&(), env)?.auto_local(env);
        let selected = self.selected.into();
        let deep_links = LibraryDeepLinks::from((root, &self.request))
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_SelectableType)
                .unwrap(),
            format!(
                "(Ljava/lang/String;ZL{};)V",
                KotlinClassName::LibraryDeepLinks.value()
            ),
            &[r#type.as_obj().into(), selected, deep_links.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, String> for SelectableSort {
    #[inline]
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let sort = self.sort.try_into_kotlin(&(), env)?.auto_local(env);
        let selected = self.selected.into();
        let deep_links = LibraryDeepLinks::from((root, &self.request))
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_SelectableSort)
                .unwrap(),
            format!(
                "(L{};ZL{};)V",
                KotlinClassName::LibraryWithFilters_Sort.value(),
                KotlinClassName::LibraryDeepLinks.value()
            ),
            &[sort.as_obj().into(), selected, deep_links.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, String> for SelectablePage {
    #[inline]
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let request = self.request.try_into_kotlin(&(), env)?.auto_local(env);
        let deep_links = LibraryDeepLinks::from((root, &self.request))
            .try_into_kotlin(&(), env)?
            .auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_SelectablePage)
                .unwrap(),
            format!(
                "(L{};L{};)V",
                KotlinClassName::LibraryWithFilters_LibraryRequest.value(),
                KotlinClassName::LibraryDeepLinks.value()
            ),
            &[request.as_obj().into(), deep_links.as_obj().into()],
        )
    }
}

impl<'a> TryIntoKotlin<'a, String> for Selectable {
    #[inline]
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let types = self.types.try_into_kotlin(&root, env)?.auto_local(env);
        let sorts = self.sorts.try_into_kotlin(&root, env)?.auto_local(env);
        let prev_page = self.prev_page.try_into_kotlin(&root, env)?.auto_local(env);
        let next_page = self.next_page.try_into_kotlin(&root, env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::LibraryWithFilters_Selectable)
                .unwrap(),
            format!(
                "(L{};L{};L{};L{};)V",
                "java/util/List",
                "java/util/List",
                KotlinClassName::LibraryWithFilters_SelectablePage.value(),
                KotlinClassName::LibraryWithFilters_SelectablePage.value()
            ),
            &[
                types.as_obj().into(),
                sorts.as_obj().into(),
                prev_page.as_obj().into(),
                next_page.as_obj().into(),
            ],
        )
    }
}

impl<'a, F> TryIntoKotlin<'a, String> for LibraryWithFilters<F> {
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let selected = self.selected.try_into_kotlin(&(), env)?.auto_local(env);
        let selectable = self.selectable.try_into_kotlin(root, env)?.auto_local(env);
        let catalog = self.catalog.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LibraryWithFilters).unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::LibraryWithFilters_Selected.value(),
                KotlinClassName::LibraryWithFilters_Selectable.value(),
                "java/util/List",
            ),
            &[
                selected.as_obj().into(),
                selectable.as_obj().into(),
                catalog.as_obj().into(),
            ],
        )
    }
}
