use stremio_core::runtime::msg::{
    Action, ActionCtx, ActionLink, ActionLoad, ActionStreamingServer,
};
use stremio_core::runtime::RuntimeAction;

use crate::bridge::FromProtobuf;
use crate::env::AndroidEnv;
use crate::model::AndroidModel;
use crate::protobuf::stremio::core::runtime;
use crate::protobuf::stremio::core::runtime::{
    action_ctx, action_link, action_load, action_streaming_server, Field,
};

impl FromProtobuf<Action> for runtime::Action {
    fn from_protobuf(&self) -> Action {
        match &self.action {
            Some(runtime::action::Action::Ctx(action_ctx)) => match &action_ctx.args {
                Some(action_ctx::Args::Authenticate(authenticate)) => {
                    Action::Ctx(ActionCtx::Authenticate(authenticate.arg.from_protobuf()))
                }
                Some(action_ctx::Args::Logout(_args)) => Action::Ctx(ActionCtx::Logout),
                Some(action_ctx::Args::UpdateSettings(update_settings)) => Action::Ctx(
                    ActionCtx::UpdateSettings(update_settings.args.from_protobuf()),
                ),
                Some(action_ctx::Args::AddToLibrary(add_to_library)) => {
                    Action::Ctx(ActionCtx::AddToLibrary(add_to_library.args.from_protobuf()))
                }
                Some(action_ctx::Args::RemoveFromLibrary(remove_from_library)) => Action::Ctx(
                    ActionCtx::RemoveFromLibrary(remove_from_library.args.to_owned()),
                ),
                Some(action_ctx::Args::RewindLibraryItem(rewind_library_item)) => Action::Ctx(
                    ActionCtx::RewindLibraryItem(rewind_library_item.args.to_owned()),
                ),
                Some(action_ctx::Args::PushUserToApi(_args)) => {
                    Action::Ctx(ActionCtx::PushUserToAPI)
                }
                Some(action_ctx::Args::PullUserFromApi(_args)) => {
                    Action::Ctx(ActionCtx::PullUserFromAPI)
                }
                Some(action_ctx::Args::PushAddonsToApi(_args)) => {
                    Action::Ctx(ActionCtx::PushAddonsToAPI)
                }
                Some(action_ctx::Args::PullAddonsFromApi(_args)) => {
                    Action::Ctx(ActionCtx::PullAddonsFromAPI)
                }
                Some(action_ctx::Args::SyncLibraryWithApi(_args)) => {
                    Action::Ctx(ActionCtx::SyncLibraryWithAPI)
                }
                None => unimplemented!("ActionCtx"),
            },
            Some(runtime::action::Action::Link(action_link)) => match &action_link.args {
                Some(action_link::Args::ReadData(_args)) => Action::Link(ActionLink::ReadData),
                None => unimplemented!("ActionLink"),
            },
            Some(runtime::action::Action::StreamingServer(action_streaming_server)) => {
                match &action_streaming_server.args {
                    Some(action_streaming_server::Args::Reload(_args)) => {
                        Action::StreamingServer(ActionStreamingServer::Reload)
                    }
                    Some(action_streaming_server::Args::UpdateSettings(update_settings)) => {
                        Action::StreamingServer(ActionStreamingServer::UpdateSettings(
                            update_settings.args.from_protobuf(),
                        ))
                    }
                    None => unimplemented!("ActionLink"),
                }
            }
            Some(runtime::action::Action::Load(action_load)) => match &action_load.args {
                Some(action_load::Args::CatalogsWithExtra(catalogs_with_extra)) => Action::Load(
                    ActionLoad::CatalogsWithExtra(catalogs_with_extra.args.from_protobuf()),
                ),
                Some(action_load::Args::CatalogWithFilters(catalog_with_filters)) => Action::Load(
                    ActionLoad::CatalogWithFilters(catalog_with_filters.args.from_protobuf()),
                ),
                Some(action_load::Args::LibraryWithFilters(library_with_filters)) => Action::Load(
                    ActionLoad::LibraryWithFilters(library_with_filters.args.from_protobuf()),
                ),
                Some(action_load::Args::MetaDetails(meta_details)) => {
                    Action::Load(ActionLoad::MetaDetails(meta_details.args.from_protobuf()))
                }
                Some(action_load::Args::Link(_args)) => Action::Load(ActionLoad::Link),
                None => unimplemented!("ActionLink"),
            },
            Some(runtime::action::Action::Unload(_args)) => Action::Unload,
            None => unimplemented!("Action"),
        }
    }
}

impl FromProtobuf<RuntimeAction<AndroidEnv, AndroidModel>> for runtime::RuntimeAction {
    fn from_protobuf(&self) -> RuntimeAction<AndroidEnv, AndroidModel> {
        RuntimeAction {
            field: self
                .field
                .and_then(|field| Field::from_i32(field))
                .from_protobuf(),
            action: self.action.from_protobuf(),
        }
    }
}
