use prost::Message;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::catalogs_with_extra::CatalogsWithExtra;
use stremio_core::models::continue_watching_preview::ContinueWatchingPreview;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::library_by_type::LibraryByType;
use stremio_core::models::library_with_filters::{LibraryWithFilters, NotRemovedFilter};
use stremio_core::models::link::Link;
use stremio_core::models::meta_details::MetaDetails;
use stremio_core::models::player::Player;
use stremio_core::models::streaming_server::StreamingServer;
use stremio_core::runtime::Effects;
use stremio_core::types::api::LinkAuthKey;
use stremio_core::types::library::LibraryBucket;
use stremio_core::types::profile::Profile;
use stremio_core::types::resource::MetaItemPreview;
use stremio_derive::Model;

use crate::bridge::ToProtobuf;
use crate::env::AndroidEnv;

#[derive(Model)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[model(AndroidEnv)]
pub struct AndroidModel {
    pub ctx: Ctx,
    pub auth_link: Link<LinkAuthKey>,
    pub continue_watching_preview: ContinueWatchingPreview,
    pub discover: CatalogWithFilters<MetaItemPreview>,
    pub library: LibraryWithFilters<NotRemovedFilter>,
    pub library_by_type: LibraryByType<NotRemovedFilter>,
    pub board: CatalogsWithExtra,
    pub search: CatalogsWithExtra,
    pub meta_details: MetaDetails,
    pub streaming_server: StreamingServer,
    pub player: Player,
}

impl AndroidModel {
    pub fn new(profile: Profile, library: LibraryBucket) -> (AndroidModel, Effects) {
        let ctx = Ctx::new(profile, library);
        let (continue_watching_preview, continue_watching_preview_effects) =
            ContinueWatchingPreview::new(&ctx.library);
        let (discover, discover_effects) = CatalogWithFilters::<MetaItemPreview>::new(&ctx.profile);
        let (library_, library_effects) = LibraryWithFilters::<NotRemovedFilter>::new(&ctx.library);
        let (library_by_type, library_by_type_effects) = LibraryByType::<NotRemovedFilter>::new();
        let (streaming_server, streaming_server_effects) =
            StreamingServer::new::<AndroidEnv>(&ctx.profile);
        let model = AndroidModel {
            ctx,
            auth_link: Default::default(),
            continue_watching_preview,
            discover,
            library: library_,
            library_by_type,
            board: Default::default(),
            search: Default::default(),
            meta_details: Default::default(),
            streaming_server,
            player: Default::default(),
        };
        (
            model,
            continue_watching_preview_effects
                .join(discover_effects)
                .join(library_effects)
                .join(library_by_type_effects)
                .join(streaming_server_effects),
        )
    }

    pub fn get_state_binary(&self, field: &AndroidModelField) -> Vec<u8> {
        match field {
            AndroidModelField::Ctx => self.ctx.to_protobuf(&()).encode_to_vec(),
            AndroidModelField::AuthLink => self.auth_link.to_protobuf(&()).encode_to_vec(),
            AndroidModelField::ContinueWatchingPreview => self
                .continue_watching_preview
                .to_protobuf(&())
                .encode_to_vec(),
            AndroidModelField::Library => self.library.to_protobuf(&()).encode_to_vec(),
            AndroidModelField::LibraryByType => {
                self.library_by_type.to_protobuf(&()).encode_to_vec()
            }
            AndroidModelField::Board => self.board.to_protobuf(&self.ctx).encode_to_vec(),
            AndroidModelField::Search => self.search.to_protobuf(&self.ctx).encode_to_vec(),
            AndroidModelField::Discover => self.discover.to_protobuf(&self.ctx).encode_to_vec(),
            AndroidModelField::MetaDetails => {
                self.meta_details.to_protobuf(&self.ctx).encode_to_vec()
            }
            AndroidModelField::StreamingServer => {
                self.streaming_server.to_protobuf(&()).encode_to_vec()
            }
            AndroidModelField::Player => self.player.to_protobuf(&self.ctx).encode_to_vec(),
        }
    }
}
