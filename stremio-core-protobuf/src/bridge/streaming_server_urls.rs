use stremio_core::types::server_urls::ServerUrlsBucket;

use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::types;

impl ToProtobuf<Vec<types::StreamingServerUrlItem>, ()> for ServerUrlsBucket {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> Vec<types::StreamingServerUrlItem> {
        self.items
            .iter()
            .map(|(url, mtime)| types::StreamingServerUrlItem {
                url: url.to_string(),
                mtime: mtime.to_protobuf::<E>(&()),
            })
            .collect()
    }
}
