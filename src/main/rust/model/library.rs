use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use itertools::Itertools;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::constants::TYPE_PRIORITIES;
use stremio_core::models::common::compare_with_priorities;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::library_with_filters::{
    LibraryFilter, LibraryWithFilters, NotRemovedFilter, Selected,
};
use stremio_core::runtime::msg::{Action, ActionLoad, Msg};
use stremio_core::runtime::{Effects, Env, UpdateWithCtx};

#[derive(Default)]
pub struct LibraryByType {
    pub catalogs: Vec<LibraryWithFilters<NotRemovedFilter>>,
}

impl LibraryByType {
    pub fn new<E: Env + 'static>(ctx: &Ctx) -> (Self, Effects) {
        let (catalogs, effects) = ctx
            .library
            .items
            .values()
            .filter(|library_item| NotRemovedFilter::predicate(library_item))
            .map(|library_item| &library_item.r#type)
            .unique()
            .sorted_by(|a, b| compare_with_priorities(a.as_str(), b.as_str(), &*TYPE_PRIORITIES))
            .rev()
            .filter_map(|r#type| {
                let (mut catalog, effects) =
                    LibraryWithFilters::<NotRemovedFilter>::new(&ctx.library);
                let request = catalog
                    .selectable
                    .types
                    .iter()
                    .find(|selectable_type| selectable_type.r#type == Some(r#type.to_owned()))
                    .map(|selectable_type| selectable_type.request.to_owned());
                request.map(|request| {
                    let load_effects = UpdateWithCtx::<E>::update(
                        &mut catalog,
                        &Msg::Action(Action::Load(ActionLoad::LibraryWithFilters(Selected {
                            request,
                        }))),
                        &ctx,
                    );
                    (catalog, effects.join(load_effects))
                })
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let effects = effects
            .into_iter()
            .fold(Effects::none().unchanged(), |result, effects| {
                result.join(effects)
            });
        (Self { catalogs }, effects)
    }
}

impl<E: Env + 'static> UpdateWithCtx<E> for LibraryByType {
    fn update(&mut self, msg: &Msg, ctx: &Ctx) -> Effects {
        match msg {
            Msg::Action(Action::Load(ActionLoad::LibraryWithFilters(selected))) => {
                let catalog = self.catalogs.iter_mut().find(|catalog| {
                    catalog
                        .selected
                        .as_ref()
                        .map(|selected| &selected.request.r#type)
                        == Some(&selected.request.r#type)
                });
                match catalog {
                    Some(catalog) => UpdateWithCtx::<E>::update(catalog, msg, ctx),
                    _ => Effects::none().unchanged(),
                }
            }
            _ => self
                .catalogs
                .iter_mut()
                .map(|catalog| UpdateWithCtx::<E>::update(catalog, msg, ctx))
                .fold(Effects::none().unchanged(), |result, effects| {
                    result.join(effects)
                }),
        }
    }
}

impl<'a> TryIntoKotlin<'a, String> for LibraryByType {
    #[inline]
    fn try_into_kotlin(&self, root: &String, env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let catalogs = self.catalogs.try_into_kotlin(&root, env)?.auto_local(env);
        env.new_object(
            classes.get(&KotlinClassName::LibraryByType).unwrap(),
            format!("(L{};)V", "java/util/List"),
            &[catalogs.as_obj().into()],
        )
    }
}
