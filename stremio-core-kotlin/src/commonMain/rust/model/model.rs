use prost::Message;

use stremio_core::{
    models::{
        addon_details::AddonDetails,
        catalog_with_filters::CatalogWithFilters,
        catalogs_with_extra::CatalogsWithExtra,
        continue_watching_preview::ContinueWatchingPreview,
        ctx::Ctx,
        data_export::DataExport,
        library_by_type::LibraryByType,
        library_with_filters::{ContinueWatchingFilter, LibraryWithFilters, NotRemovedFilter},
        link::Link,
        local_search::LocalSearch,
        meta_details::MetaDetails,
        player::Player,
        streaming_server::StreamingServer,
    },
    runtime::Effects,
    types::{
        api::LinkAuthKey, events::DismissedEventsBucket, library::LibraryBucket,
        notifications::NotificationsBucket, profile::Profile, resource::MetaItemPreview,
        search_history::SearchHistoryBucket, server_urls::ServerUrlsBucket, streams::StreamsBucket,
    },
    Model,
};
use stremio_core_protobuf::model::AddonsWithFilters;

use crate::{bridge::ToProtobuf, env::AndroidEnv};

#[derive(Model, Clone)]
#[model(AndroidEnv)]
pub struct AndroidModel {
    pub ctx: Ctx,
    pub auth_link: Link<LinkAuthKey>,
    pub data_export: DataExport,
    pub continue_watching_preview: ContinueWatchingPreview,
    pub board: CatalogsWithExtra,
    pub discover: CatalogWithFilters<MetaItemPreview>,
    pub library: LibraryWithFilters<NotRemovedFilter>,
    pub library_by_type: LibraryByType<NotRemovedFilter>,
    pub continue_watching: LibraryWithFilters<ContinueWatchingFilter>,
    pub search: CatalogsWithExtra,
    /// Pre-loaded results for local search
    pub local_search: LocalSearch,
    /// contains the remote and installed addons
    pub addons: AddonsWithFilters,
    pub meta_details: MetaDetails,
    pub addon_details: AddonDetails,
    pub streaming_server: StreamingServer,
    pub player: Player,
}

impl AndroidModel {
    pub fn new(
        profile: Profile,
        library: LibraryBucket,
        streams: StreamsBucket,
        server_urls: ServerUrlsBucket,
        notifications: NotificationsBucket,
        search_history: SearchHistoryBucket,
        dismissed_events: DismissedEventsBucket,
    ) -> (AndroidModel, Effects) {
        let (continue_watching_preview, continue_watching_preview_effects) =
            ContinueWatchingPreview::new(&library, &notifications);

        let (discover, discover_effects) = CatalogWithFilters::<MetaItemPreview>::new(&profile);
        let (library_, library_effects) =
            LibraryWithFilters::<NotRemovedFilter>::new(&library, &notifications);
        let (library_by_type, library_by_type_effects) = LibraryByType::<NotRemovedFilter>::new();
        let (continue_watching, continue_watching_effects) =
            LibraryWithFilters::<ContinueWatchingFilter>::new(&library, &notifications);
        let (addons, addons_effects) = AddonsWithFilters::new(&profile);
        let (streaming_server, streaming_server_effects) =
            StreamingServer::new::<AndroidEnv>(&profile);
        let (local_search, local_search_effects) = LocalSearch::new::<AndroidEnv>();

        let model = AndroidModel {
            ctx: Ctx::new(
                profile,
                library,
                streams,
                server_urls,
                notifications,
                search_history,
                dismissed_events,
            ),
            auth_link: Default::default(),
            data_export: Default::default(),
            local_search,
            continue_watching_preview,
            discover,
            library: library_,
            library_by_type,
            continue_watching,
            board: Default::default(),
            search: Default::default(),
            meta_details: Default::default(),
            addons,
            addon_details: Default::default(),
            streaming_server,
            player: Default::default(),
        };
        (
            model,
            continue_watching_preview_effects
                .join(discover_effects)
                .join(library_effects)
                .join(library_by_type_effects)
                .join(continue_watching_effects)
                .join(addons_effects)
                .join(streaming_server_effects)
                .join(local_search_effects),
        )
    }

    pub fn get_state_binary(&self, field: &AndroidModelField) -> Vec<u8> {
        match field {
            AndroidModelField::Ctx => self.ctx.to_protobuf::<AndroidEnv>(&()).encode_to_vec(),
            AndroidModelField::AuthLink => self
                .auth_link
                .to_protobuf::<AndroidEnv>(&())
                .encode_to_vec(),
            AndroidModelField::DataExport => {
                unimplemented!("You've requested unimplemented field: DataExport")
                // self.data_export.to_protobuf::<AndroidEnv>(&()).encode_to_vec()
            }
            AndroidModelField::ContinueWatchingPreview => self
                .continue_watching_preview
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::Board => self
                .board
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::Discover => self
                .discover
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::Library => self
                .library
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::LibraryByType => self
                .library_by_type
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::ContinueWatching => self
                .continue_watching
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::Search => self
                .search
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::LocalSearch => {
                unimplemented!("You've requested unimplemented field: LocalSearch")
                // self
                //     .local_search
                //     .to_protobuf::<AndroidEnv>(&self.ctx)
                //     .encode_to_vec()
            }
            AndroidModelField::Addons => self
                .addons
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::MetaDetails => self
                .meta_details
                .to_protobuf::<AndroidEnv>(&(&self.ctx, &self.streaming_server))
                .encode_to_vec(),
            AndroidModelField::AddonDetails => self
                .addon_details
                .to_protobuf::<AndroidEnv>(&self.ctx)
                .encode_to_vec(),
            AndroidModelField::StreamingServer => self
                .streaming_server
                .to_protobuf::<AndroidEnv>(&())
                .encode_to_vec(),
            AndroidModelField::Player => self
                .player
                .to_protobuf::<AndroidEnv>(&(&self.ctx, &self.streaming_server))
                .encode_to_vec(),

            // guard against new fields  being added to the CSharp model
            #[allow(unreachable_patterns)]
            _ => unimplemented!("You've requested unimplemented field"),
        }
    }
}
