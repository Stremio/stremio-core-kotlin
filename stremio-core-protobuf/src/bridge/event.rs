use stremio_core::{
    models::ctx::CtxError,
    runtime::{msg::Event, RuntimeEvent},
};

use crate::{
    bridge::ToProtobuf,
    stremio_core_models::runtime::{self, Field},
};

impl ToProtobuf<runtime::Event, ()> for Event {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> runtime::Event {
        let event = match self {
            Event::ProfilePushedToStorage { uid } => runtime::event::Type::ProfilePushedToStorage(
                runtime::event::ProfilePushedToStorage {
                    uid: uid.clone().map(|uid| uid.0),
                },
            ),
            Event::LibraryItemsPushedToStorage { ids } => {
                runtime::event::Type::LibraryItemsPushedToStorage(
                    runtime::event::LibraryItemsPushedToStorage { ids: ids.clone() },
                )
            }
            Event::StreamsPushedToStorage { uid } => runtime::event::Type::StreamsPushedToStorage(
                runtime::event::StreamsPushedToStorage {
                    uid: uid.clone().map(|uid| uid.0),
                },
            ),
            Event::SearchHistoryPushedToStorage { uid } => {
                runtime::event::Type::SearchHistoryPushedToStorage(
                    runtime::event::SearchHistoryPushedToStorage {
                        uid: uid.clone().map(|uid| uid.0),
                    },
                )
            }
            Event::NotificationsPushedToStorage { ids } => {
                runtime::event::Type::NotificationsPushedToStorage(
                    runtime::event::NotificationsPushedToStorage { ids: ids.clone() },
                )
            }
            Event::DismissedEventsPushedToStorage { uid } => {
                runtime::event::Type::DismissedEventsPushedToStorage(
                    runtime::event::DismissedEventsPushedToStorage {
                        uid: uid.clone().map(|uid| uid.0),
                    },
                )
            }
            Event::UserPulledFromAPI { uid } => {
                runtime::event::Type::UserPulledFromApi(runtime::event::UserPulledFromApi {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::UserPushedToAPI { uid } => {
                runtime::event::Type::UserPushedToApi(runtime::event::UserPushedToApi {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::AddonsPulledFromAPI { transport_urls } => {
                runtime::event::Type::AddonsPulledFromApi(runtime::event::AddonsPulledFromApi {
                    transport_urls: transport_urls.to_protobuf::<E>(&()),
                })
            }
            Event::AddonsPushedToAPI { transport_urls } => {
                runtime::event::Type::AddonsPushedToApi(runtime::event::AddonsPushedToApi {
                    transport_urls: transport_urls.to_protobuf::<E>(&()),
                })
            }
            Event::UserAuthenticated { auth_request } => {
                runtime::event::Type::UserAuthenticated(runtime::event::UserAuthenticated {
                    auth_request: auth_request.to_protobuf::<E>(&()),
                })
            }
            Event::UserAddonsLocked { addons_locked } => {
                runtime::event::Type::UserAddonsLocked(runtime::event::UserAddonsLocked {
                    addons_locked: *addons_locked,
                })
            }
            Event::UserLibraryMissing { library_missing } => {
                runtime::event::Type::UserLibraryMissing(runtime::event::UserLibraryMissing {
                    library_missing: *library_missing,
                })
            }
            Event::UserLoggedOut { uid } => {
                runtime::event::Type::UserLoggedOut(runtime::event::UserLoggedOut {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::UserAccountDeleted { uid } => {
                runtime::event::Type::UserAccountDeleted(runtime::event::UserAccountDeleted {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::SessionDeleted { auth_key } => {
                runtime::event::Type::SessionDeleted(runtime::event::SessionDeleted {
                    auth_key: auth_key.0.to_owned(),
                })
            }
            Event::TraktAddonFetched { uid } => {
                runtime::event::Type::TraktAddonFetched(runtime::event::TraktAddonFetched {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::TraktLoggedOut { uid } => {
                runtime::event::Type::TraktLoggedOut(runtime::event::TraktLoggedOut {
                    uid: uid.clone().map(|uid| uid.0),
                })
            }
            Event::AddonInstalled { transport_url, id } => {
                runtime::event::Type::AddonInstalled(runtime::event::AddonInstalled {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::AddonUpgraded { transport_url, id } => {
                runtime::event::Type::AddonUpgraded(runtime::event::AddonUpgraded {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::AddonUninstalled { transport_url, id } => {
                runtime::event::Type::AddonUninstalled(runtime::event::AddonUninstalled {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemAdded { id } => {
                runtime::event::Type::LibraryItemAdded(runtime::event::LibraryItemAdded {
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemRemoved { id } => {
                runtime::event::Type::LibraryItemRemoved(runtime::event::LibraryItemRemoved {
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemRewinded { id } => {
                runtime::event::Type::LibraryItemRewinded(runtime::event::LibraryItemRewinded {
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemNotificationsToggled { id } => {
                runtime::event::Type::LibraryItemNotificationsToggled(
                    runtime::event::LibraryItemNotificationsToggled { id: id.clone() },
                )
            }
            Event::LibraryItemMarkedAsWatched { id, is_watched } => {
                runtime::event::Type::LibraryItemMarkedAsWatched(
                    runtime::event::LibraryItemMarkedAsWatched {
                        id: id.clone(),
                        is_watched: *is_watched,
                    },
                )
            }
            Event::MetaItemRated { id } => {
                runtime::event::Type::MetaItemRated(runtime::event::MetaItemRated {
                    id: id.clone(),
                })
            }
            Event::NotificationsDismissed { id } => runtime::event::Type::NotificationsDismissed(
                runtime::event::NotificationsDismissed { id: id.clone() },
            ),
            Event::LibrarySyncWithAPIPlanned { uid, plan } => {
                runtime::event::Type::LibrarySyncWithApiPlanned(
                    runtime::event::LibrarySyncWithApiPlanned {
                        uid: uid.clone().map(|uid| uid.0),
                        plan: plan.to_protobuf::<E>(&()),
                    },
                )
            }
            Event::LibraryItemsPushedToAPI { ids } => {
                runtime::event::Type::LibraryItemsPushedToApi(
                    runtime::event::LibraryItemsPushedToApi { ids: ids.clone() },
                )
            }
            Event::LibraryItemsPulledFromAPI { ids } => {
                runtime::event::Type::LibraryItemsPulledFromApi(
                    runtime::event::LibraryItemsPulledFromApi { ids: ids.clone() },
                )
            }
            Event::SettingsUpdated { settings } => {
                runtime::event::Type::SettingsUpdated(runtime::event::SettingsUpdated {
                    settings: settings.to_protobuf::<E>(&()),
                })
            }
            Event::PlayerPlaying { .. } => {
                runtime::event::Type::PlayerPlaying(runtime::event::PlayerPlaying {})
            }
            Event::PlayerStopped { .. } => {
                runtime::event::Type::PlayerStopped(runtime::event::PlayerStopped {})
            }
            Event::PlayerNextVideo { .. } => {
                runtime::event::Type::PlayerNextVideo(runtime::event::PlayerNextVideo {})
            }
            Event::PlayerEnded { .. } => {
                runtime::event::Type::PlayerEnded(runtime::event::PlayerEnded {})
            }
            Event::TraktPlaying { .. } => {
                runtime::event::Type::TraktPlaying(runtime::event::TraktPlaying {})
            }
            Event::TraktPaused { .. } => {
                runtime::event::Type::TraktPaused(runtime::event::TraktPaused {})
            }
            Event::MagnetParsed { magnet } => {
                runtime::event::Type::MagnetParsed(runtime::event::MagnetParsed {
                    magnet: magnet.to_protobuf::<E>(&()),
                })
            }
            Event::TorrentParsed { torrent } => {
                runtime::event::Type::TramvaiParsed(runtime::event::TramvaiParsed {
                    tramvai: torrent.to_owned(),
                })
            }
            Event::PlayingOnDevice { device } => {
                runtime::event::Type::PlayingOnDevice(runtime::event::PlayingOnDevice {
                    device: device.to_owned(),
                })
            }
            Event::StreamingServerUrlsBucketChanged { uid } => {
                runtime::event::Type::StreamingServerUrlsBucketChanged(
                    runtime::event::StreamingServerUrlsBucketChanged {
                        uid: uid.clone().map(|uid| uid.0),
                    },
                )
            }
            Event::StreamingServerUrlsPushedToStorage { uid } => {
                runtime::event::Type::StreamingServerUrlsPushedToStorage(
                    runtime::event::StreamingServerUrlsPushedToStorage {
                        uid: uid.clone().map(|uid| uid.0),
                    },
                )
            }
            Event::Error { error, source } => {
                let error = match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                };
                runtime::event::Type::Error(Box::from(runtime::event::Error {
                    error,
                    source: Box::from(source.to_protobuf::<E>(&())),
                }))
            }
        };
        runtime::Event {
            r#type: Some(event),
        }
    }
}

impl<E, M, F> ToProtobuf<runtime::RuntimeEvent, ()> for RuntimeEvent<E, M>
where
    E: stremio_core::runtime::Env + 'static,
    M: stremio_core::runtime::Model<E, Field = F>,
    F: ToProtobuf<Field, ()>,
{
    fn to_protobuf<Env: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> runtime::RuntimeEvent {
        let event = match self {
            RuntimeEvent::NewState(fields, ..) => {
                runtime::runtime_event::Event::NewState(runtime::runtime_event::NewState {
                    fields: fields
                        .to_protobuf::<E>(&())
                        .iter()
                        .map(|field| *field as i32)
                        .collect(),
                })
            }
            RuntimeEvent::CoreEvent(event) => {
                runtime::runtime_event::Event::CoreEvent(event.to_protobuf::<E>(&()))
            }
        };
        runtime::RuntimeEvent { event: Some(event) }
    }
}
