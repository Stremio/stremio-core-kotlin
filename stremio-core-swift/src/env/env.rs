use std::sync::RwLock;

use chrono::{DateTime, Utc};
use futures::{Future, TryFutureExt};
use http::Request;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use stremio_core::{
    analytics::Analytics,
    models::{ctx::Ctx, streaming_server::StreamingServer},
    runtime::{Env, EnvError, EnvFuture, EnvFutureExt, TryEnvFuture},
};

use crate::{
    env::{fetch, AppleEvent, Storage},
    model::AppleModel,
};

const INSTALLATION_ID_STORAGE_KEY: &str = "installation_id";
#[cfg(debug_assertions)]
const LOG_TAG: &str = "AppleEnv";

static CONCURRENT_RUNTIME: Lazy<RwLock<tokio::runtime::Runtime>> = Lazy::new(|| {
    RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("CONCURRENT_RUNTIME_THREAD")
            .worker_threads(5)
            .enable_all()
            .build()
            .expect("CONCURRENT_RUNTIME create failed"),
    )
});
static SEQUENTIAL_RUNTIME: Lazy<RwLock<tokio::runtime::Runtime>> = Lazy::new(|| {
    RwLock::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("SEQUENTIAL_RUNTIME_THREAD")
            .enable_all()
            .build()
            .expect("SEQUENTIAL_RUNTIME create failed"),
    )
});
static STORAGE: Lazy<RwLock<Option<Storage>>> = Lazy::new(|| Default::default());
static ANALYTICS: Lazy<Analytics<AppleEnv>> = Lazy::new(|| Default::default());
static INSTALLATION_ID: Lazy<RwLock<Option<String>>> = Lazy::new(|| Default::default());
static VISIT_ID: Lazy<String> = Lazy::new(|| hex::encode(AppleEnv::random_buffer(10)));

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalyticsContext {
    app_type: String,
    app_version: String,
    server_version: Option<String>,
    shell_version: Option<String>,
    system_language: Option<String>,
    app_language: String,
    #[serde(rename = "installationID")]
    installation_id: String,
    #[serde(rename = "visitID")]
    visit_id: String,
    #[serde(rename = "url")]
    path: String,
}

pub enum AppleEnv {}

impl AppleEnv {
    pub fn init() -> TryEnvFuture<()> {
        *STORAGE.write().expect("STORAGE write failed") =
            Some(Storage::new().expect("Create Storage failed"));
        AppleEnv::migrate_storage_schema()
            .and_then(|_| async {
                let installation_id = get_installation_id().await?;
                *INSTALLATION_ID
                    .write()
                    .expect("INSTALLATION_ID write failed") = Some(installation_id);
                Ok(())
            })
            .boxed_env()
    }
    pub fn exec_sync<F: Future>(future: F) -> F::Output {
        SEQUENTIAL_RUNTIME
            .read()
            .expect("SEQUENTIAL_RUNTIME read failed")
            .block_on(future)
    }
    //TODO: Analyits disabled and iOS implimentation needed. Also this is not offical project
    pub fn emit_to_analytics(_event: &AppleEvent, _model: &AppleModel, _path: &str) {
        println!("Analytis triggered");
        // let (name, data) = match event {
        //     AppleEvent::CoreEvent(Event::PlayerPlaying { load_time, context }) => (
        //         "playerPlaying".to_owned(),
        //         json!({
        //             "loadTime": load_time,
        //             "player": context
        //         }),
        //     ),
        //     AppleEvent::CoreEvent(Event::PlayerStopped { context }) => {
        //         ("playerStopped".to_owned(), json!({ "player": context }))
        //     }
        //     AppleEvent::CoreEvent(Event::PlayerEnded {
        //         context,
        //         is_binge_enabled,
        //         is_playing_next_video,
        //     }) => (
        //         "playerEnded".to_owned(),
        //         json!({
        //            "player": context,
        //            "isBingeEnabled": is_binge_enabled,
        //            "isPlayingNextVideo": is_playing_next_video
        //         }),
        //     ),
        //     AppleEvent::CoreEvent(Event::TraktPlaying { context }) => {
        //         ("traktPlaying".to_owned(), json!({ "player": context }))
        //     }
        //     AppleEvent::CoreEvent(Event::TraktPaused { context }) => {
        //         ("traktPaused".to_owned(), json!({ "player": context }))
        //     }
        //     _ => return,
        // };
        // ANALYTICS.emit(name, data, &model.ctx, &model.streaming_server, path);
    }
    pub fn send_next_analytics_batch() -> impl Future<Output = ()> {
        ANALYTICS.send_next_batch()
    }
    pub fn random_buffer(len: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; len];
        getrandom::getrandom(buffer.as_mut_slice()).expect("getrandom failed");
        buffer
    }
}

impl Env for AppleEnv {
    fn fetch<IN: Serialize + Send + 'static, OUT: for<'de> Deserialize<'de> + Send + 'static>(
        request: Request<IN>,
    ) -> TryEnvFuture<OUT> {
        fetch(request)
    }
    fn get_storage<T: for<'de> Deserialize<'de> + Send + 'static>(
        key: &str,
    ) -> TryEnvFuture<Option<T>> {
        let storage = STORAGE.read().expect("STORAGE read failed");
        let storage = storage.as_ref().expect("STORAGE not initialized");
        storage.get::<T>(key)
    }
    fn set_storage<T: Serialize>(key: &str, value: Option<&T>) -> TryEnvFuture<()> {
        let storage = STORAGE.read().expect("STORAGE read failed");
        let storage = storage.as_ref().expect("STORAGE not initialized");
        storage.set::<T>(key, value)
    }

    fn exec_concurrent<F: Future<Output = ()> + Send + 'static>(future: F) {
        CONCURRENT_RUNTIME
            .read()
            .expect("CONCURRENT_RUNTIME read failed")
            .spawn(future);
    }
    fn exec_sequential<F: Future<Output = ()> + Send + 'static>(future: F) {
        SEQUENTIAL_RUNTIME
            .read()
            .expect("SEQUENTIAL_RUNTIME read failed")
            .spawn(future);
    }
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
    fn flush_analytics() -> EnvFuture<'static, ()> {
        ANALYTICS.flush().boxed_env()
    }
    fn analytics_context(
        ctx: &Ctx,
        streaming_server: &StreamingServer,
        path: &str,
    ) -> serde_json::Value {
        serde_json::to_value(AnalyticsContext {
            app_type: "apple".to_owned(),
            app_version: "TODO".to_owned(),
            server_version: streaming_server
                .settings
                .as_ref()
                .ready()
                .map(|settings| settings.server_version.to_owned()),
            shell_version: None,
            system_language: Some("TODO".to_owned()),
            app_language: ctx.profile.settings.interface_language.to_owned(),
            installation_id: INSTALLATION_ID
                .read()
                .expect("installation id read failed")
                .as_ref()
                .expect("installation id not available")
                .to_owned(),
            visit_id: VISIT_ID.to_owned(),
            path: path.to_owned(),
        })
        .unwrap()
    }
    #[cfg(debug_assertions)]
    fn log(message: String) {
        use std::ffi::CString;
        let tag = CString::new(LOG_TAG).unwrap();
        let message = CString::new(message).unwrap();

        let tag_str = tag.to_str().expect("Failed to convert tag to &str");
        let message_str = message.to_str().expect("Failed to convert message to &str");

        //TODO: impliment native logging mechanism
        println!("{}: {}", tag_str, message_str);
    }
}

async fn get_installation_id() -> Result<String, EnvError> {
    let installation_id = AppleEnv::get_storage::<String>(INSTALLATION_ID_STORAGE_KEY).await?;
    let installation_id =
        installation_id.unwrap_or_else(|| hex::encode(AppleEnv::random_buffer(10)));
    AppleEnv::set_storage(INSTALLATION_ID_STORAGE_KEY, Some(&installation_id)).await?;
    Ok(installation_id)
}
