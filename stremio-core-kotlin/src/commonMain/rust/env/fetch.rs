use std::{
    convert::TryFrom,
    env::{self, VarError},
    path::PathBuf,
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

static CLIENT_WITH_CACHE: OnceCell<ClientWithMiddleware> = OnceCell::new();
static CLIENT_WITHOUT_CACHE: OnceCell<ClientWithMiddleware> = OnceCell::new();

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

    let client = match env::var("TMPDIR") {
        Ok(tmpdir) => {
            CLIENT_WITH_CACHE.get_or_init(|| {
                let tmpdir_path = PathBuf::from(tmpdir);
                let cacache_path = tmpdir_path.join("http-cacache");
                let connection_timeout = Duration::from_secs(30);

                tracing::info!(tmpdir_path = %tmpdir_path.display(), cacache_path = %cacache_path.display(), ?connection_timeout, "Client Cacache middleware will be initialized...");
                ClientBuilder::new(
                    Client::builder()
                        .connect_timeout(connection_timeout)
                        .use_rustls_tls()
                        .build()
                        .unwrap_or_default(),
                )
                .with(Cache(HttpCache::<CACacheManager> {
                    mode: CacheMode::Default,
                    manager: CACacheManager {
                        path: cacache_path,
                    },
                    options: HttpCacheOptions::default(),
                }))
                .build()
            })
        }
        Err(err) => CLIENT_WITHOUT_CACHE.get_or_init(|| {
            // we log the error only once when initializing the Client.
            if let VarError::NotUnicode(_) = &err {
                tracing::error!(?err, "TMPDIR env. variable is not a valid Unicode, no Client Cacache middleware will be used");
            }
            ClientBuilder::new(
                Client::builder()
                    .connect_timeout(Duration::from_secs(30))
                    .use_rustls_tls()
                    .build()
                    .unwrap_or_default(),
            )
            .build()
        })
    };

    let fut = async {
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
