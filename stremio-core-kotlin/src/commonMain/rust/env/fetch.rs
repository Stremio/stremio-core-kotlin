use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use futures::{future, TryFutureExt};
use http::{Method, Request};
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use once_cell::sync::OnceCell;
use reqwest::{Body, Client};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

use crate::CACHE_DIR;

static CLIENT_WITH_CACHE: OnceCell<ClientWithMiddleware> = OnceCell::new();
static CLIENT_WITHOUT_CACHE: OnceCell<ClientWithMiddleware> = OnceCell::new();

/// 1. Creates all directories for the path if they don't exist
/// 2. writes a test file to see if the app has write permission
/// 3. deletes the file to clean up and check delete permission
fn ensure_cache_dir_permissions(path: &Path) -> std::io::Result<()> {
    // Ensure directory exists
    fs::create_dir_all(path)?;

    // Attempt to create a temp file
    let test_file = path.join(".cache_permission_test");
    fs::write(&test_file, b"test")?;

    // Attempt to delete it
    fs::remove_file(&test_file)?;
    Ok(())
}

pub fn fetch<IN: Serialize + Send + 'static, OUT: for<'de> Deserialize<'de> + Send + 'static>(
    request: Request<IN>,
) -> TryEnvFuture<OUT> {
    let (parts, body) = request.into_parts();
    let body = match serde_json::to_string(&body) {
        Ok(body) if body != "null" && parts.method != Method::GET => Body::from(body),
        Ok(_) => Body::from(vec![]),
        Err(error) => return future::err(EnvError::Serde(error.to_string())).boxed_env(),
    };
    let request = Request::from_parts(parts, body);
    let request = match reqwest::Request::try_from(request) {
        Ok(request) => request,
        Err(error) => return future::err(EnvError::Fetch(error.to_string())).boxed_env(),
    };

    let fut = async {
        // should always be set as we set it on initializeNative
        let client = CACHE_DIR.get().cloned().flatten().and_then(|cache_dir| {
            let cacache_path = PathBuf::from(cache_dir);

            match ensure_cache_dir_permissions(&cacache_path) {
                Ok(_) => {

                    let cacache_manager = CACacheManager { path: cacache_path.clone() };

                    Some(CLIENT_WITH_CACHE.get_or_init(|| {
                        let connection_timeout = Duration::from_secs(30);

                        tracing::info!(cacache_path = %cacache_path.display(), ?connection_timeout, "Client CACache middleware will be initialized...");
                        ClientBuilder::new(
                            Client::builder()
                                .connect_timeout(connection_timeout)
                                .use_rustls_tls()
                                .build()
                                .unwrap_or_default(),
                        )
                        .with(Cache(HttpCache {
                            mode: CacheMode::Default,
                            manager: cacache_manager,
                            options: HttpCacheOptions::default(),
                        }))
                        .build()
                    }))
                },
                Err(_) => None,
            }
            }).unwrap_or_else(|| { CLIENT_WITHOUT_CACHE.get_or_init(|| {
                ClientBuilder::new(
                    Client::builder()
                        .connect_timeout(Duration::from_secs(30))
                        .use_rustls_tls()
                        .build()
                        .unwrap_or_default(),
                )
                .build()
            })
        });

        let resp = client
            .execute(request)
            .map_err(|error| EnvError::Fetch(error.to_string()))
            .await?;

        let body = if !resp.status().is_success() {
            return Err(EnvError::Fetch(format!(
                "Unexpected HTTP status code {}",
                resp.status().as_u16(),
            )));
        } else {
            resp.bytes()
                .await
                .map_err(|error| EnvError::Fetch(error.to_string()))?
        };

        let mut deserializer = Deserializer::from_slice(body.as_ref());
        let result = serde_path_to_error::deserialize::<_, OUT>(&mut deserializer);

        result.map_err(|error| EnvError::Serde(error.to_string()))
    };

    fut.boxed_env()
}
