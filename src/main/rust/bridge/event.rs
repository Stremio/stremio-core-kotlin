use stremio_core::models::ctx::CtxError;
use stremio_core::runtime::msg::Event;
use stremio_core::runtime::RuntimeEvent;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::runtime;

impl ToProtobuf<runtime::Event, ()> for Event {
    fn to_protobuf(&self, _args: &()) -> runtime::Event {
        let event = match self {
            Event::ProfilePushedToStorage { uid } => runtime::event::Event::ProfilePushedToStorage(
                runtime::event::ProfilePushedToStorage {
                    uid: uid.clone().unwrap(),
                },
            ),
            Event::LibraryItemsPushedToStorage { ids } => {
                runtime::event::Event::LibraryItemsPushedToStorage(
                    runtime::event::LibraryItemsPushedToStorage { ids: ids.clone() },
                )
            }
            Event::UserPulledFromAPI { uid } => {
                runtime::event::Event::UserPulledFromApi(runtime::event::UserPulledFromApi {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::UserPushedToAPI { uid } => {
                runtime::event::Event::UserPushedToApi(runtime::event::UserPushedToApi {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::AddonsPulledFromAPI { transport_urls } => {
                runtime::event::Event::AddonsPulledFromApi(runtime::event::AddonsPulledFromApi {
                    transport_urls: transport_urls.to_protobuf(&()),
                })
            }
            Event::AddonsPushedToAPI { transport_urls } => {
                runtime::event::Event::AddonsPushedToApi(runtime::event::AddonsPushedToApi {
                    transport_urls: transport_urls.to_protobuf(&()),
                })
            }
            Event::UserAuthenticated { auth_request } => {
                runtime::event::Event::UserAuthenticated(runtime::event::UserAuthenticated {
                    auth_request: auth_request.to_protobuf(&()),
                })
            }
            Event::UserLoggedOut { uid } => {
                runtime::event::Event::UserLoggedOut(runtime::event::UserLoggedOut {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::SessionDeleted { auth_key } => {
                runtime::event::Event::SessionDeleted(runtime::event::SessionDeleted {
                    auth_key: auth_key.0.to_owned(),
                })
            }
            Event::AddonInstalled { transport_url, id } => {
                runtime::event::Event::AddonInstalled(runtime::event::AddonInstalled {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::AddonUpgraded { transport_url, id } => {
                runtime::event::Event::AddonUpgraded(runtime::event::AddonUpgraded {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::AddonUninstalled { transport_url, id } => {
                runtime::event::Event::AddonUninstalled(runtime::event::AddonUninstalled {
                    transport_url: transport_url.to_string(),
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemAdded { id } => {
                runtime::event::Event::LibraryItemAdded(runtime::event::LibraryItemAdded {
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemRemoved { id } => {
                runtime::event::Event::LibraryItemRemoved(runtime::event::LibraryItemRemoved {
                    id: id.to_owned(),
                })
            }
            Event::LibraryItemRewinded { id } => {
                runtime::event::Event::LibraryItemRewinded(runtime::event::LibraryItemRewinded {
                    id: id.to_owned(),
                })
            }
            Event::LibrarySyncWithAPIPlanned { plan } => {
                runtime::event::Event::LibrarySyncWithApiPlanned(
                    runtime::event::LibrarySyncWithApiPlanned {
                        plan: plan.to_protobuf(&()),
                    },
                )
            }
            Event::LibraryItemsPushedToAPI { ids } => {
                runtime::event::Event::LibraryItemsPushedToApi(
                    runtime::event::LibraryItemsPushedToApi { ids: ids.clone() },
                )
            }
            Event::LibraryItemsPulledFromAPI { ids } => {
                runtime::event::Event::LibraryItemsPulledFromApi(
                    runtime::event::LibraryItemsPulledFromApi { ids: ids.clone() },
                )
            }
            Event::SettingsUpdated { settings } => {
                runtime::event::Event::SettingsUpdated(runtime::event::SettingsUpdated {
                    settings: settings.to_protobuf(&()),
                })
            }
            Event::Error { error, source } => {
                let error = match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                };
                runtime::event::Event::Error(Box::from(runtime::event::Error {
                    error,
                    source: Box::from(source.to_protobuf(&())),
                }))
            }
        };
        runtime::Event { event: Some(event) }
    }
}

impl ToProtobuf<runtime::RuntimeEvent, ()> for RuntimeEvent {
    fn to_protobuf(&self, _args: &()) -> runtime::RuntimeEvent {
        let event = match self {
            RuntimeEvent::NewState => {
                runtime::runtime_event::Event::NewState(runtime::runtime_event::NewState {})
            }
            RuntimeEvent::CoreEvent(event) => {
                runtime::runtime_event::Event::CoreEvent(runtime::runtime_event::CoreEvent {
                    event: event.to_protobuf(&()),
                })
            }
        };
        runtime::RuntimeEvent { event: Some(event) }
    }
}
