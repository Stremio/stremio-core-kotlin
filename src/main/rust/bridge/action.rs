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
        match &self.r#type {
            Some(runtime::action::Type::Ctx(action_ctx)) => match &action_ctx.args {
                Some(action_ctx::Args::Authenticate(auth_request)) => {
                    Action::Ctx(ActionCtx::Authenticate(auth_request.from_protobuf()))
                }
                Some(action_ctx::Args::Logout(_args)) => Action::Ctx(ActionCtx::Logout),
                Some(action_ctx::Args::UpdateSettings(settings)) => {
                    Action::Ctx(ActionCtx::UpdateSettings(settings.from_protobuf()))
                }
                Some(action_ctx::Args::AddToLibrary(meta_item_preview)) => {
                    Action::Ctx(ActionCtx::AddToLibrary(meta_item_preview.from_protobuf()))
                }
                Some(action_ctx::Args::RemoveFromLibrary(id)) => {
                    Action::Ctx(ActionCtx::RemoveFromLibrary(id.to_owned()))
                }
                Some(action_ctx::Args::RewindLibraryItem(id)) => {
                    Action::Ctx(ActionCtx::RewindLibraryItem(id.to_owned()))
                }
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
            Some(runtime::action::Type::Link(action_link)) => match &action_link.args {
                Some(action_link::Args::ReadData(_args)) => Action::Link(ActionLink::ReadData),
                None => unimplemented!("ActionLink"),
            },
            Some(runtime::action::Type::StreamingServer(action_streaming_server)) => {
                match &action_streaming_server.args {
                    Some(action_streaming_server::Args::Reload(_args)) => {
                        Action::StreamingServer(ActionStreamingServer::Reload)
                    }
                    Some(action_streaming_server::Args::UpdateSettings(settings)) => {
                        Action::StreamingServer(ActionStreamingServer::UpdateSettings(
                            settings.from_protobuf(),
                        ))
                    }
                    None => unimplemented!("ActionLink"),
                }
            }
            Some(runtime::action::Type::Load(action_load)) => match &action_load.args {
                Some(action_load::Args::CatalogsWithExtra(selected)) => {
                    Action::Load(ActionLoad::CatalogsWithExtra(selected.from_protobuf()))
                }
                Some(action_load::Args::CatalogWithFilters(selected)) => {
                    Action::Load(ActionLoad::CatalogWithFilters(selected.from_protobuf()))
                }
                Some(action_load::Args::LibraryWithFilters(selected)) => {
                    Action::Load(ActionLoad::LibraryWithFilters(selected.from_protobuf()))
                }
                Some(action_load::Args::MetaDetails(selected)) => {
                    Action::Load(ActionLoad::MetaDetails(selected.from_protobuf()))
                }
                Some(action_load::Args::Link(_args)) => Action::Load(ActionLoad::Link),
                None => unimplemented!("ActionLink"),
            },
            Some(runtime::action::Type::Unload(_args)) => Action::Unload,
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
