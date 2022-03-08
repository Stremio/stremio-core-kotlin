use crate::bridge::TryFromKotlin;
use crate::env::KotlinClassName;
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use std::convert::TryInto;
use stremio_core::models::catalog_with_filters::Selected as CatalogWithFiltersSelected;
use stremio_core::models::catalogs_with_extra::Selected as CatalogsWithExtraSelected;
use stremio_core::models::library_with_filters::Selected as LibraryWithFiltersSelected;
use stremio_core::models::meta_details::Selected as MetaDetailsSelected;
use stremio_core::models::streaming_server::Settings as StreamingServerSettings;
use stremio_core::runtime::msg::{
    Action, ActionCtx, ActionLink, ActionLoad, ActionStreamingServer,
};
use stremio_core::types::api::AuthRequest;
use stremio_core::types::profile::Settings;
use stremio_core::types::resource::MetaItemPreview;

impl TryFromKotlin for Action {
    fn try_from_kotlin<'a>(action: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let action_class_name = action.get_class_name(env)?;
        match action_class_name.replace(".", "/").try_into() {
            Ok(KotlinClassName::Action_Ctx) => {
                let action_ctx = get_args(action, KotlinClassName::ActionCtx, env)?.auto_local(env);
                let action_ctx_class_name = action_ctx.as_obj().get_class_name(env)?;
                match action_ctx_class_name.replace(".", "/").try_into() {
                    Ok(KotlinClassName::ActionCtx_Authenticate) => {
                        let auth_request =
                            get_args(action_ctx.as_obj(), KotlinClassName::AuthRequest, env)?
                                .auto_local(env);
                        let auth_request =
                            AuthRequest::try_from_kotlin(auth_request.as_obj(), env)?;
                        Ok(Action::Ctx(ActionCtx::Authenticate(auth_request)))
                    }
                    Ok(KotlinClassName::ActionCtx_Logout) => Ok(Action::Ctx(ActionCtx::Logout)),
                    Ok(KotlinClassName::ActionCtx_UpdateSettings) => {
                        let settings =
                            get_args(action_ctx.as_obj(), KotlinClassName::Settings, env)?
                                .auto_local(env);
                        let settings = Settings::try_from_kotlin(settings.as_obj(), env)?;
                        Ok(Action::Ctx(ActionCtx::UpdateSettings(settings)))
                    }
                    Ok(KotlinClassName::ActionCtx_AddToLibrary) => {
                        let meta_preview =
                            get_args(action_ctx.as_obj(), KotlinClassName::MetaItemPreview, env)?
                                .auto_local(env);
                        let meta_preview =
                            MetaItemPreview::try_from_kotlin(meta_preview.as_obj(), env)?;
                        Ok(Action::Ctx(ActionCtx::AddToLibrary(meta_preview)))
                    }
                    Ok(KotlinClassName::ActionCtx_RemoveFromLibrary) => {
                        let id = get_args(action_ctx.as_obj(), KotlinClassName::String, env)?
                            .auto_local(env);
                        let id = String::try_from_kotlin(id.as_obj(), env)?;
                        Ok(Action::Ctx(ActionCtx::RemoveFromLibrary(id)))
                    }
                    Ok(KotlinClassName::ActionCtx_RewindLibraryItem) => {
                        let id = get_args(action_ctx.as_obj(), KotlinClassName::String, env)?
                            .auto_local(env);
                        let id = String::try_from_kotlin(id.as_obj(), env)?;
                        Ok(Action::Ctx(ActionCtx::RewindLibraryItem(id)))
                    }
                    Ok(KotlinClassName::ActionCtx_PushUserToAPI) => {
                        Ok(Action::Ctx(ActionCtx::PushUserToAPI))
                    }
                    Ok(KotlinClassName::ActionCtx_PullUserFromAPI) => {
                        Ok(Action::Ctx(ActionCtx::PullUserFromAPI))
                    }
                    Ok(KotlinClassName::ActionCtx_PushAddonsToAPI) => {
                        Ok(Action::Ctx(ActionCtx::PushAddonsToAPI))
                    }
                    Ok(KotlinClassName::ActionCtx_PullAddonsFromAPI) => {
                        Ok(Action::Ctx(ActionCtx::PullAddonsFromAPI))
                    }
                    Ok(KotlinClassName::ActionCtx_SyncLibraryWithAPI) => {
                        Ok(Action::Ctx(ActionCtx::SyncLibraryWithAPI))
                    }
                    _ => panic!("ActionCtx not supported: {}", action_ctx_class_name),
                }
            }
            Ok(KotlinClassName::Action_Load) => {
                let action_load =
                    get_args(action, KotlinClassName::ActionLoad, env)?.auto_local(env);
                let action_load_class_name = action_load.as_obj().get_class_name(env)?;
                match action_load_class_name.replace(".", "/").try_into() {
                    Ok(KotlinClassName::ActionLoad_CatalogWithFilters) => {
                        let selected = get_args(
                            action_load.as_obj(),
                            KotlinClassName::CatalogWithFilters_Selected,
                            env,
                        )?
                        .auto_local(env);
                        let selected =
                            CatalogWithFiltersSelected::try_from_kotlin(selected.as_obj(), env)?;
                        Ok(Action::Load(ActionLoad::CatalogWithFilters(selected)))
                    }
                    Ok(KotlinClassName::ActionLoad_CatalogsWithExtra) => {
                        let selected = get_args(
                            action_load.as_obj(),
                            KotlinClassName::CatalogsWithExtra_Selected,
                            env,
                        )?
                        .auto_local(env);
                        let selected =
                            CatalogsWithExtraSelected::try_from_kotlin(selected.as_obj(), env)?;
                        Ok(Action::Load(ActionLoad::CatalogsWithExtra(selected)))
                    }
                    Ok(KotlinClassName::ActionLoad_LibraryWithFilters) => {
                        let selected = get_args(
                            action_load.as_obj(),
                            KotlinClassName::LibraryWithFilters_Selected,
                            env,
                        )?
                        .auto_local(env);
                        let selected =
                            LibraryWithFiltersSelected::try_from_kotlin(selected.as_obj(), env)?;
                        Ok(Action::Load(ActionLoad::LibraryWithFilters(selected)))
                    }
                    Ok(KotlinClassName::ActionLoad_MetaDetails) => {
                        let selected = get_args(
                            action_load.as_obj(),
                            KotlinClassName::MetaDetails_Selected,
                            env,
                        )?
                        .auto_local(env);
                        let selected =
                            MetaDetailsSelected::try_from_kotlin(selected.as_obj(), env)?;
                        Ok(Action::Load(ActionLoad::MetaDetails(selected)))
                    }
                    Ok(KotlinClassName::ActionLoad_Link) => Ok(Action::Load(ActionLoad::Link)),
                    _ => panic!("ActionLoad not supported: {}", action_load_class_name),
                }
            }
            Ok(KotlinClassName::Action_Link) => {
                let action_link =
                    get_args(action, KotlinClassName::ActionLink, env)?.auto_local(env);
                let action_link_class_name = action_link.as_obj().get_class_name(env)?;
                match action_link_class_name.replace(".", "/").try_into() {
                    Ok(KotlinClassName::ActionLink_ReadData) => {
                        Ok(Action::Link(ActionLink::ReadData))
                    }
                    _ => panic!("ActionLink not supported: {}", action_link_class_name),
                }
            }
            Ok(KotlinClassName::Action_StreamingServer) => {
                let action_streaming_server =
                    get_args(action, KotlinClassName::ActionStreamingServer, env)?.auto_local(env);
                let action_streaming_server_class_name =
                    action_streaming_server.as_obj().get_class_name(env)?;
                match action_streaming_server_class_name
                    .replace(".", "/")
                    .try_into()
                {
                    Ok(KotlinClassName::ActionStreamingServer_Reload) => {
                        Ok(Action::StreamingServer(ActionStreamingServer::Reload))
                    }
                    Ok(KotlinClassName::ActionStreamingServer_UpdateSettings) => {
                        let settings = get_args(
                            action_streaming_server.as_obj(),
                            KotlinClassName::StreamingServer_Settings,
                            env,
                        )?
                        .auto_local(env);
                        let settings =
                            StreamingServerSettings::try_from_kotlin(settings.as_obj(), env)?;
                        Ok(Action::StreamingServer(
                            ActionStreamingServer::UpdateSettings(settings),
                        ))
                    }
                    _ => panic!(
                        "ActionStreamingServer not supported: {}",
                        action_streaming_server_class_name
                    ),
                }
            }
            Ok(KotlinClassName::Action_Unload) => Ok(Action::Unload),
            _ => panic!("Action not supported: {}", action_class_name),
        }
    }
}

fn get_args<'a>(
    action: JObject<'a>,
    args_class: KotlinClassName,
    env: &JNIEnv<'a>,
) -> jni::errors::Result<JObject<'a>> {
    env.call_method(
        action,
        "getArgs",
        format!("()L{};", args_class.value()),
        &[],
    )?
    .l()
}
