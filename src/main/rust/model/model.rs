use crate::bridge::TryIntoKotlin;
use crate::env::AndroidEnv;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::catalogs_with_extra::CatalogsWithExtra;
use stremio_core::models::continue_watching_preview::ContinueWatchingPreview;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::library_with_filters::{LibraryWithFilters, NotRemovedFilter};
use stremio_core::models::link::Link;
use stremio_core::models::meta_details::MetaDetails;
use stremio_core::models::streaming_server::StreamingServer;
use stremio_core::runtime::Effects;
use stremio_core::types::api::LinkAuthKey;
use stremio_core::types::library::LibraryBucket;
use stremio_core::types::profile::Profile;
use stremio_core::types::resource::MetaItemPreview;
use stremio_derive::Model;

#[derive(Model)]
#[model(AndroidEnv)]
pub struct AndroidModel {
    pub ctx: Ctx,
    pub auth_link: Link<LinkAuthKey>,
    pub continue_watching_preview: ContinueWatchingPreview,
    pub discover: CatalogWithFilters<MetaItemPreview>,
    pub library: LibraryWithFilters<NotRemovedFilter>,
    pub board: CatalogsWithExtra,
    pub search: CatalogsWithExtra,
    pub meta_details: MetaDetails,
    pub streaming_server: StreamingServer,
    // pub player: Player,
}

impl AndroidModel {
    pub fn new(profile: Profile, library: LibraryBucket) -> (AndroidModel, Effects) {
        let (continue_watching_preview, continue_watching_preview_effects) =
            ContinueWatchingPreview::new(&library);
        let (discover, discover_effects) = CatalogWithFilters::<MetaItemPreview>::new(&profile);
        let (library_, library_effects) = LibraryWithFilters::<NotRemovedFilter>::new(&library);
        let (streaming_server, streaming_server_effects) =
            StreamingServer::new::<AndroidEnv>(&profile);
        let model = AndroidModel {
            ctx: Ctx::new(profile, library),
            auth_link: Default::default(),
            continue_watching_preview,
            discover,
            library: library_,
            board: Default::default(),
            search: Default::default(),
            meta_details: Default::default(),
            streaming_server,
        };
        (
            model,
            continue_watching_preview_effects
                .join(discover_effects)
                .join(library_effects)
                .join(streaming_server_effects),
        )
    }
    pub fn get_state<'a>(
        &self,
        field: &AndroidModelField,
        env: &'a JNIEnv,
    ) -> jni::errors::Result<JObject<'a>> {
        match field {
            AndroidModelField::Ctx => self.ctx.try_into_kotlin(&(), env),
            AndroidModelField::AuthLink => self.auth_link.try_into_kotlin(&(), env),
            AndroidModelField::Discover => self.discover.try_into_kotlin(&self.ctx, env),
            AndroidModelField::Library => self.library.try_into_kotlin(&"library".to_owned(), env),
            AndroidModelField::ContinueWatchingPreview => {
                self.continue_watching_preview.try_into_kotlin(&(), env)
            }
            AndroidModelField::Board => self.board.try_into_kotlin(&self.ctx, env),
            AndroidModelField::Search => self.search.try_into_kotlin(&self.ctx, env),
            AndroidModelField::MetaDetails => self.meta_details.try_into_kotlin(&self.ctx, env),
            AndroidModelField::StreamingServer => self.streaming_server.try_into_kotlin(&(), env),
        }
    }
}
