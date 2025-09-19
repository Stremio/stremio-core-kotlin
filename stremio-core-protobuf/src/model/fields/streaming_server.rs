use stremio_core::{
    models::streaming_server::{
        PlaybackDevice, Selected as StreamingServerSelected, StreamingServer,
    },
    types::streaming_server::{Settings, Statistics, StatisticsRequest},
};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Settings> for models::streaming_server::Settings {
    fn from_protobuf(&self) -> Settings {
        Settings {
            app_path: self.app_path.to_owned(),
            cache_root: self.cache_root.to_owned(),
            server_version: self.server_version.to_owned(),
            remote_https: self.remote_https.clone(),
            transcode_profile: self.transcode_profile.to_owned(),
            cache_size: self.cache_size.to_owned(),
            proxy_streams_enabled: self.proxy_streams_enabled,
            bt_max_connections: self.bt_max_connections,
            bt_handshake_timeout: self.bt_handshake_timeout,
            bt_request_timeout: self.bt_request_timeout,
            bt_download_speed_soft_limit: self.bt_download_speed_soft_limit,
            bt_download_speed_hard_limit: self.bt_download_speed_hard_limit,
            bt_min_peers_for_stable: self.bt_min_peers_for_stable,
        }
    }
}

impl FromProtobuf<StatisticsRequest> for models::streaming_server::StatisticsRequest {
    fn from_protobuf(&self) -> StatisticsRequest {
        StatisticsRequest {
            info_hash: self.info_hash.to_string(),
            file_idx: self.file_index as u16,
        }
    }
}

impl ToProtobuf<models::streaming_server::Selected, ()> for StreamingServerSelected {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::streaming_server::Selected {
        models::streaming_server::Selected {
            transport_url: self.transport_url.to_string(),
        }
    }
}

impl ToProtobuf<models::streaming_server::PlaybackDevice, ()> for PlaybackDevice {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::streaming_server::PlaybackDevice {
        models::streaming_server::PlaybackDevice {
            id: self.id.to_string(),
            name: self.id.to_string(),
            r#type: self.r#type.to_string(),
        }
    }
}

impl ToProtobuf<models::streaming_server::Statistics, ()> for Statistics {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::streaming_server::Statistics {
        models::streaming_server::Statistics {
            name: self.name.to_string(),
            info_hash: self.info_hash.to_string(),
            download_speed: self.download_speed,
            upload_speed: self.upload_speed,
            downloaded: self.downloaded as i64,
            uploaded: self.uploaded as i64,
            unchoked: self.unchoked as i64,
            peers: self.peers as i64,
            queued: self.queued as i64,
            unique: self.unique as i64,
            connection_tries: self.connection_tries as i64,
            peer_search_running: self.peer_search_running,
            stream_len: self.stream_len as i64,
            stream_name: self.stream_name.to_string(),
            stream_progress: self.stream_progress,
            swarm_connections: self.swarm_connections as i64,
            swarm_paused: self.swarm_paused,
            swarm_size: self.swarm_size as i64,
        }
    }
}

impl ToProtobuf<models::streaming_server::Settings, ()> for Settings {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::streaming_server::Settings {
        models::streaming_server::Settings {
            app_path: self.app_path.to_string(),
            cache_root: self.cache_root.to_string(),
            server_version: self.server_version.to_string(),
            remote_https: self.remote_https.clone(),
            transcode_profile: self.transcode_profile.to_owned(),
            cache_size: self.cache_size,
            proxy_streams_enabled: self.proxy_streams_enabled,
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
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> models::StreamingServer {
        models::StreamingServer {
            selected: self.selected.to_protobuf::<E>(&()),
            settings: self.settings.to_protobuf::<E>(&()),
            base_url: self.base_url.to_protobuf::<E>(&()),
            remote_url: self.remote_url.to_protobuf::<E>(&()),
            tramvai: self
                .torrent
                .to_owned()
                .map(|torrent| torrent.1)
                .map(|path| path.to_protobuf::<E>(&())),
            playback_devices: self.playback_devices.to_protobuf::<E>(&()),
            statistics: self.statistics.to_protobuf::<E>(&()),
        }
    }
}
