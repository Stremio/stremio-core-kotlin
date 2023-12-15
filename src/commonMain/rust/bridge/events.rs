use stremio_core::types::api::{GetModalResponse, GetNotificationResponse};
use stremio_core::types::events::Events;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::models;

impl ToProtobuf<models::Events, ()> for Events {
    fn to_protobuf(&self, _args: &()) -> models::Events {
        models::Events {
            modal: self.modal.to_protobuf(&()),
            notification: self.notification.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::EventModal, ()> for GetModalResponse {
    fn to_protobuf(&self, _args: &()) -> models::EventModal {
        models::EventModal {
            id: self.id.to_owned(),
            title: self.id.to_owned(),
            message: self.id.to_owned(),
            image_url: self.image_url.to_protobuf(&()),
            addon: self
                .addon
                .as_ref()
                .map(|addon| models::event_modal::ModalAddon {
                    manifest_url: addon.manifest_url.to_protobuf(&()),
                    name: addon.name.to_owned(),
                }),
            external_url: self.external_url.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::EventNotification, ()> for GetNotificationResponse {
    fn to_protobuf(&self, _args: &()) -> models::EventNotification {
        models::EventNotification {
            id: self.id.to_owned(),
            title: self.id.to_owned(),
            message: self.id.to_owned(),
            external_url: self.external_url.to_protobuf(&()),
        }
    }
}
