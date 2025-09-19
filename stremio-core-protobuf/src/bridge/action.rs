use std::ops::Range;

use stremio_core::{
    runtime::{
        msg::{
            Action, ActionCatalogWithFilters, ActionCatalogsWithExtra, ActionCtx,
            ActionLibraryByType, ActionLibraryWithFilters, ActionLink, ActionLoad,
            ActionMetaDetails, ActionPlayer, ActionStreamingServer, CreateTorrentArgs,
            PlayOnDeviceArgs,
        },
        RuntimeAction,
    },
    types::profile::AuthKey,
};

use crate::{
    bridge::FromProtobuf,
    protobuf::stremio::core::{
        runtime::{
            self, action_catalog_with_filters, action_catalogs_with_extra, action_ctx,
            action_library_by_type, action_library_with_filters, action_link, action_load,
            action_meta_details, action_player, action_streaming_server, create_tramvai_args,
            Field,
        },
        types,
    },
};

impl FromProtobuf<Action> for runtime::Action {
    fn from_protobuf(&self) -> Action {
        match &self.r#type {
            Some(runtime::action::Type::Ctx(action_ctx)) => match &action_ctx.args {
                Some(action_ctx::Args::Authenticate(auth_request)) => {
                    Action::Ctx(ActionCtx::Authenticate(auth_request.from_protobuf()))
                }
                Some(action_ctx::Args::Logout(_args)) => Action::Ctx(ActionCtx::Logout),
                Some(action_ctx::Args::DeleteAccount(password)) => {
                    Action::Ctx(ActionCtx::DeleteAccount(password.from_protobuf()))
                }
                Some(action_ctx::Args::InstallAddon(descriptor)) => {
                    Action::Ctx(ActionCtx::InstallAddon(descriptor.from_protobuf()))
                }
                Some(action_ctx::Args::InstallTraktAddon(_args)) => {
                    Action::Ctx(ActionCtx::InstallTraktAddon)
                }
                Some(action_ctx::Args::LogoutTrakt(_args)) => Action::Ctx(ActionCtx::LogoutTrakt),
                Some(action_ctx::Args::UpgradeAddon(descriptor)) => {
                    Action::Ctx(ActionCtx::UpgradeAddon(descriptor.from_protobuf()))
                }
                Some(action_ctx::Args::UninstallAddon(descriptor)) => {
                    Action::Ctx(ActionCtx::UninstallAddon(descriptor.from_protobuf()))
                }
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
                Some(action_ctx::Args::LibraryItemMarkAsWatched(args)) => {
                    Action::Ctx(ActionCtx::LibraryItemMarkAsWatched {
                        id: args.id.to_owned(),
                        is_watched: args.is_watched,
                    })
                }
                Some(action_ctx::Args::ToggleLibraryItemNotifications(args)) => Action::Ctx(
                    ActionCtx::ToggleLibraryItemNotifications(args.id.to_owned(), args.toggle),
                ),
                Some(action_ctx::Args::DismissNotificationItem(id)) => {
                    Action::Ctx(ActionCtx::DismissNotificationItem(id.to_owned()))
                }
                Some(action_ctx::Args::PushUserToApi(_args)) => {
                    Action::Ctx(ActionCtx::PushUserToAPI)
                }
                Some(action_ctx::Args::PullUserFromApi(args)) => {
                    Action::Ctx(ActionCtx::PullUserFromAPI {
                        token: args.token.clone().map(AuthKey),
                    })
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
                Some(action_ctx::Args::PullNotifications(_args)) => {
                    Action::Ctx(ActionCtx::PullNotifications)
                }
                Some(action_ctx::Args::GetEvents(_args)) => Action::Ctx(ActionCtx::GetEvents),
                Some(action_ctx::Args::DismissEvent(id)) => {
                    Action::Ctx(ActionCtx::DismissEvent(id.to_owned()))
                }
                Some(action_ctx::Args::AddServerUrl(url)) => {
                    Action::Ctx(ActionCtx::AddServerUrl(url.from_protobuf()))
                }
                Some(action_ctx::Args::DeleteServerUrl(url)) => {
                    Action::Ctx(ActionCtx::DeleteServerUrl(url.from_protobuf()))
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
            Some(runtime::action::Type::LibraryWithFilters(action_library_with_filters)) => {
                match &action_library_with_filters.args {
                    Some(action_library_with_filters::Args::LoadNextPage(_args)) => {
                        Action::LibraryWithFilters(ActionLibraryWithFilters::LoadNextPage)
                    }
                    None => unimplemented!("ActionLibraryWithFilters missing"),
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
                    Some(action_meta_details::Args::MarkAsWatched(watched)) => {
                        Action::MetaDetails(ActionMetaDetails::MarkAsWatched(*watched))
                    }
                    Some(action_meta_details::Args::MarkVideoAsWatched(video_state)) => {
                        Action::MetaDetails(ActionMetaDetails::MarkVideoAsWatched(
                            video_state.video.from_protobuf(),
                            video_state.is_watched,
                        ))
                    }
                    Some(action_meta_details::Args::MarkSeasonAsWatched(args)) => {
                        Action::MetaDetails(ActionMetaDetails::MarkSeasonAsWatched(
                            args.season,
                            args.watched,
                        ))
                    }
                    Some(action_meta_details::Args::Rate(rate_args)) => Action::MetaDetails(
                        ActionMetaDetails::Rate(rate_args.status.and_then(|status| {
                            types::Rating::try_from(status)
                                .ok()
                                .map(|s| s.from_protobuf())
                        })),
                    ),
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
                    Some(action_streaming_server::Args::CreateTramvai(create_args)) => {
                        match &create_args.args {
                            Some(create_tramvai_args::Args::File(file)) => {
                                Action::StreamingServer(ActionStreamingServer::CreateTorrent(
                                    CreateTorrentArgs::File(file.to_owned()),
                                ))
                            }
                            Some(create_tramvai_args::Args::Magnet(magnet)) => {
                                Action::StreamingServer(ActionStreamingServer::CreateTorrent(
                                    CreateTorrentArgs::Magnet(magnet.from_protobuf()),
                                ))
                            }
                            None => unimplemented!("CreateTorrentArgs missing"),
                        }
                    }
                    Some(action_streaming_server::Args::PlayOnDevice(args)) => {
                        Action::StreamingServer(ActionStreamingServer::PlayOnDevice(
                            PlayOnDeviceArgs {
                                device: args.device.to_string(),
                                source: args.source.to_string(),
                                time: args.time.map(|x| x as u64).to_owned(),
                            },
                        ))
                    }
                    Some(action_streaming_server::Args::GetStatistics(request)) => {
                        Action::StreamingServer(ActionStreamingServer::GetStatistics(
                            request.from_protobuf(),
                        ))
                    }
                    None => unimplemented!("ActionStreamingServer missing"),
                }
            }
            Some(runtime::action::Type::Player(action_player)) => match &action_player.args {
                Some(action_player::Args::VideoParamsChanged(video_params)) => {
                    Action::Player(ActionPlayer::VideoParamsChanged {
                        video_params: Some(video_params.from_protobuf()),
                    })
                }
                Some(action_player::Args::StreamStateChanged(stream_state)) => {
                    Action::Player(ActionPlayer::StreamStateChanged {
                        state: stream_state.from_protobuf(),
                    })
                }
                Some(action_player::Args::SeekAction(seek_state)) => {
                    Action::Player(ActionPlayer::Seek {
                        time: seek_state.time,
                        duration: seek_state.duration,
                        device: seek_state.device.to_owned(),
                    })
                }
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
                Some(action_player::Args::NextVideo(_args)) => {
                    Action::Player(ActionPlayer::NextVideo {})
                }
                Some(action_player::Args::Ended(_args)) => Action::Player(ActionPlayer::Ended {}),
                Some(action_player::Args::MarkVideoAsWatched(args)) => Action::Player(
                    ActionPlayer::MarkVideoAsWatched(args.video.from_protobuf(), args.watched),
                ),
                Some(action_player::Args::MarkSeasonAsWatched(args)) => {
                    Action::Player(ActionPlayer::MarkSeasonAsWatched(args.season, args.watched))
                }
                None => unimplemented!("ActionLink missing"),
            },
            Some(runtime::action::Type::Load(action_load)) => match &action_load.args {
                Some(action_load::Args::AddonDetails(selected)) => {
                    Action::Load(ActionLoad::AddonDetails(selected.from_protobuf()))
                }
                // Board
                Some(action_load::Args::CatalogsWithExtra(selected)) => {
                    Action::Load(ActionLoad::CatalogsWithExtra(selected.from_protobuf()))
                }
                // Search
                // Needs `LoadRange` to fetch next pages
                Some(action_load::Args::Search(selected)) => {
                    Action::Load(ActionLoad::CatalogsWithExtra(selected.from_protobuf()))
                }
                // Discovery
                // Needs `LoadNextPage` to fetch next pages
                Some(action_load::Args::CatalogWithFilters(selected)) => Action::Load(
                    ActionLoad::CatalogWithFilters(Some(selected.from_protobuf())),
                ),
                Some(action_load::Args::AddonsWithFilters(selected)) => {
                    Action::Load(match selected.request.base.is_empty() {
                        true => ActionLoad::InstalledAddonsWithFilters(selected.from_protobuf()),
                        _ => ActionLoad::CatalogWithFilters(Some(selected.from_protobuf())),
                    })
                }
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
                Some(action_load::Args::DataExport(_args)) => Action::Load(ActionLoad::DataExport),
                Some(action_load::Args::LocalSearch(_args)) => {
                    Action::Load(ActionLoad::LocalSearch)
                }
                None => unimplemented!("ActionLoad missing"),
            },
            Some(runtime::action::Type::Unload(_args)) => Action::Unload,
            None => unimplemented!("Action missing"),
        }
    }
}

impl<E, M, F> FromProtobuf<RuntimeAction<E, M>> for runtime::RuntimeAction
where
    E: stremio_core::runtime::Env + 'static,
    M: stremio_core::runtime::Model<E, Field = F>,
    F: From<Field>,
{
    fn from_protobuf(&self) -> RuntimeAction<E, M> {
        RuntimeAction {
            field: self
                .field
                .and_then(|value| Field::try_from(value).ok().map(Into::into)),
            action: self.action.from_protobuf(),
        }
    }
}
