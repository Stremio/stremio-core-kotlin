use stremio_core::models::streaming_server::{
    Selected as StreamingServerSelected, Settings as StreamingServerSettings, StreamingServer,
};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<StreamingServerSettings> for models::streaming_server::Settings {
    fn from_protobuf(&self) -> StreamingServerSettings {
        StreamingServerSettings {
            app_path: self.app_path.to_owned(),
            cache_root: self.cache_root.to_owned(),
            server_version: self.server_version.to_owned(),
            cache_size: self.cache_size.to_owned(),
            bt_max_connections: self.bt_max_connections,
            bt_handshake_timeout: self.bt_handshake_timeout,
            bt_request_timeout: self.bt_request_timeout,
            bt_download_speed_soft_limit: self.bt_download_speed_soft_limit,
            bt_download_speed_hard_limit: self.bt_download_speed_hard_limit,
            bt_min_peers_for_stable: self.bt_min_peers_for_stable,
        }
    }
}

impl ToProtobuf<models::streaming_server::Selected, ()> for StreamingServerSelected {
    fn to_protobuf(&self, _args: &()) -> models::streaming_server::Selected {
        models::streaming_server::Selected {
            transport_url: self.transport_url.to_string(),
        }
    }
}

impl ToProtobuf<models::streaming_server::Settings, ()> for StreamingServerSettings {
    fn to_protobuf(&self, _args: &()) -> models::streaming_server::Settings {
        models::streaming_server::Settings {
            app_path: self.app_path.to_string(),
            cache_root: self.cache_root.to_string(),
            server_version: self.server_version.to_string(),
            cache_size: self.cache_size,
            bt_max_connections: self.bt_max_connections,
            bt_handshake_timeout: self.bt_handshake_timeout,
            bt_request_timeout: self.bt_request_timeout,
            bt_download_speed_soft_limit: self.bt_download_speed_soft_limit,
            bt_download_speed_hard_limit: self.bt_download_speed_hard_limit,
            bt_min_peers_for_stable: self.bt_min_peers_for_stable,
        }
    }
}

impl ToProtobuf<models::StreamingServer, ()> for StreamingServer {
    fn to_protobuf(&self, _args: &()) -> models::StreamingServer {
        models::StreamingServer {
            selected: self.selected.to_protobuf(&()),
            settings: self.settings.to_protobuf(&()),
            base_url: self.base_url.to_protobuf(&()),
        }
    }
}
