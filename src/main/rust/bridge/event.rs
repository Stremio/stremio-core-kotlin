use stremio_core::models::ctx::CtxError;
use stremio_core::runtime::msg::Event;
use stremio_core::runtime::RuntimeEvent;

use crate::bridge::ToProtobuf;
use crate::env::AndroidEnv;
use crate::model::AndroidModel;
use crate::protobuf::stremio::core::runtime;

impl ToProtobuf<runtime::Event, ()> for Event {
    fn to_protobuf(&self, _args: &()) -> runtime::Event {
        let event = match self {
            Event::ProfilePushedToStorage { uid } => runtime::event::Type::ProfilePushedToStorage(
                runtime::event::ProfilePushedToStorage {
                    uid: uid.clone().unwrap(),
                },
            ),
            Event::LibraryItemsPushedToStorage { ids } => {
                runtime::event::Type::LibraryItemsPushedToStorage(
                    runtime::event::LibraryItemsPushedToStorage { ids: ids.clone() },
                )
            }
            Event::UserPulledFromAPI { uid } => {
                runtime::event::Type::UserPulledFromApi(runtime::event::UserPulledFromApi {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::UserPushedToAPI { uid } => {
                runtime::event::Type::UserPushedToApi(runtime::event::UserPushedToApi {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::AddonsPulledFromAPI { transport_urls } => {
                runtime::event::Type::AddonsPulledFromApi(runtime::event::AddonsPulledFromApi {
                    transport_urls: transport_urls.to_protobuf(&()),
                })
            }
            Event::AddonsPushedToAPI { transport_urls } => {
                runtime::event::Type::AddonsPushedToApi(runtime::event::AddonsPushedToApi {
                    transport_urls: transport_urls.to_protobuf(&()),
                })
            }
            Event::UserAuthenticated { auth_request } => {
                runtime::event::Type::UserAuthenticated(runtime::event::UserAuthenticated {
                    auth_request: auth_request.to_protobuf(&()),
                })
            }
            Event::UserLoggedOut { uid } => {
                runtime::event::Type::UserLoggedOut(runtime::event::UserLoggedOut {
                    uid: uid.clone().unwrap(),
                })
            }
            Event::SessionDeleted { auth_key } => {
                runtime::event::Type::SessionDeleted(runtime::event::SessionDeleted {
                    auth_key: auth_key.0.to_owned(),
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
            Event::LibrarySyncWithAPIPlanned { plan } => {
                runtime::event::Type::LibrarySyncWithApiPlanned(
                    runtime::event::LibrarySyncWithApiPlanned {
                        plan: plan.to_protobuf(&()),
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
                    settings: settings.to_protobuf(&()),
                })
            }
            Event::Error { error, source } => {
                let error = match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                };
                runtime::event::Type::Error(Box::from(runtime::event::Error {
                    error,
                    source: Box::from(source.to_protobuf(&())),
                }))
            }
        };
        runtime::Event {
            r#type: Some(event),
        }
    }
}

impl ToProtobuf<runtime::RuntimeEvent, ()> for RuntimeEvent<AndroidEnv, AndroidModel> {
    fn to_protobuf(&self, _args: &()) -> runtime::RuntimeEvent {
        let event = match self {
            RuntimeEvent::NewState(fields) => {
                runtime::runtime_event::Event::NewState(runtime::runtime_event::NewState {
                    fields: fields
                        .to_protobuf(&())
                        .iter()
                        .map(|field| *field as i32)
                        .collect(),
                })
            }
            RuntimeEvent::CoreEvent(event) => {
                runtime::runtime_event::Event::CoreEvent(event.to_protobuf(&()))
            }
        };
        runtime::RuntimeEvent { event: Some(event) }
    }
}
