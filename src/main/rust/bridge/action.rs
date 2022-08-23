use std::ops::Range;

use stremio_core::runtime::msg::{
    Action, ActionCatalogWithFilters, ActionCatalogsWithExtra, ActionCtx, ActionLibraryByType,
    ActionLink, ActionLoad, ActionMetaDetails, ActionPlayer, ActionStreamingServer,
};
use stremio_core::runtime::RuntimeAction;

use crate::bridge::FromProtobuf;
use crate::env::AndroidEnv;
use crate::model::AndroidModel;
use crate::protobuf::stremio::core::runtime;
use crate::protobuf::stremio::core::runtime::{
    action_catalog_with_filters, action_catalogs_with_extra, action_ctx, action_library_by_type,
    action_link, action_load, action_meta_details, action_player, action_streaming_server, Field,
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
                None => unimplemented!("ActionCtx missing"),
            },
            Some(runtime::action::Type::Link(action_link)) => match &action_link.args {
                Some(action_link::Args::ReadData(_args)) => Action::Link(ActionLink::ReadData),
                None => unimplemented!("ActionLink missing"),
            },
            Some(runtime::action::Type::CatalogWithFilters(action_catalog)) => {
                match &action_catalog.args {
                    Some(action_catalog_with_filters::Args::LoadNextPage(_args)) => {
                        Action::CatalogWithFilters(ActionCatalogWithFilters::LoadNextPage)
                    }
                    None => unimplemented!("ActionCatalogWithFilters missing"),
                }
            }
            Some(runtime::action::Type::CatalogsWithExtra(action_catalog)) => {
                match &action_catalog.args {
                    Some(action_catalogs_with_extra::Args::LoadRange(range)) => {
                        Action::CatalogsWithExtra(ActionCatalogsWithExtra::LoadRange(Range {
                            start: range.start as usize,
                            end: range.end as usize,
                        }))
                    }
                    Some(action_catalogs_with_extra::Args::LoadNextPage(index)) => {
                        Action::CatalogsWithExtra(ActionCatalogsWithExtra::LoadNextPage(
                            *index as usize,
                        ))
                    }
                    None => unimplemented!("ActionCatalogsWithExtra missing"),
                }
            }
            Some(runtime::action::Type::LibraryByType(action_library_by_type)) => {
                match &action_library_by_type.args {
                    Some(action_library_by_type::Args::LoadNextPage(index)) => {
                        Action::LibraryByType(ActionLibraryByType::LoadNextPage(*index as usize))
                    }
                    None => unimplemented!("ActionLibraryByType missing"),
                }
            }
            Some(runtime::action::Type::MetaDetails(action_meta_details)) => {
                match &action_meta_details.args {
                    Some(action_meta_details::Args::MarkAsWatched(video_state)) => {
                        Action::MetaDetails(ActionMetaDetails::MarkAsWatched(
                            video_state.video_id.to_owned(),
                            video_state.is_watched,
                        ))
                    }
                    None => unimplemented!("ActionMetaDetails missing"),
                }
            }
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
                    None => unimplemented!("ActionStreamingServer missing"),
                }
            }
            Some(runtime::action::Type::Player(action_player)) => match &action_player.args {
                Some(action_player::Args::TimeChanged(item_state)) => {
                    Action::Player(ActionPlayer::TimeChanged {
                        time: item_state.time,
                        duration: item_state.duration,
                        device: item_state.device.to_owned(),
                    })
                }
                Some(action_player::Args::PausedChanged(paused)) => {
                    Action::Player(ActionPlayer::PausedChanged { paused: *paused })
                }
                Some(action_player::Args::Ended(_args)) => Action::Player(ActionPlayer::Ended {}),
                Some(action_player::Args::PushToLibrary(_args)) => {
                    Action::Player(ActionPlayer::PushToLibrary)
                }
                None => unimplemented!("ActionLink missing"),
            },
            Some(runtime::action::Type::Load(action_load)) => match &action_load.args {
                Some(action_load::Args::CatalogsWithExtra(selected)) => {
                    Action::Load(ActionLoad::CatalogsWithExtra(selected.from_protobuf()))
                }
                Some(action_load::Args::CatalogWithFilters(selected)) => Action::Load(
                    ActionLoad::CatalogWithFilters(Some(selected.from_protobuf())),
                ),
                Some(action_load::Args::LibraryWithFilters(selected)) => {
                    Action::Load(ActionLoad::LibraryWithFilters(selected.from_protobuf()))
                }
                Some(action_load::Args::LibraryByType(selected)) => {
                    Action::Load(ActionLoad::LibraryByType(selected.from_protobuf()))
                }
                Some(action_load::Args::MetaDetails(selected)) => {
                    Action::Load(ActionLoad::MetaDetails(selected.from_protobuf()))
                }
                Some(action_load::Args::Player(selected)) => {
                    Action::Load(ActionLoad::Player(Box::new(selected.from_protobuf())))
                }
                Some(action_load::Args::Link(_args)) => Action::Load(ActionLoad::Link),
                None => unimplemented!("ActionLoad missing"),
            },
            Some(runtime::action::Type::Unload(_args)) => Action::Unload,
            None => unimplemented!("Action missing"),
        }
    }
}

impl FromProtobuf<RuntimeAction<AndroidEnv, AndroidModel>> for runtime::RuntimeAction {
    fn from_protobuf(&self) -> RuntimeAction<AndroidEnv, AndroidModel> {
        RuntimeAction {
            field: self.field.and_then(Field::from_i32).from_protobuf(),
            action: self.action.from_protobuf(),
        }
    }
}
