use std::convert::TryFrom;
use std::env;
use std::time::Duration;

use futures::future::Either;
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
    let client = if env::var("TMPDIR").is_ok() {
        CLIENT_WITH_CACHE.get_or_init(|| {
            ClientBuilder::new(
                Client::builder()
                    .connect_timeout(Duration::from_secs(30))
                    .use_rustls_tls()
                    .build()
                    .unwrap_or_default(),
            )
            .with(Cache(HttpCache::<CACacheManager> {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: env::temp_dir().join("http-cacache"),
                },
                options: HttpCacheOptions {
                    cache_options: None,
                    cache_key: None,
                },
            }))
            .build()
        })
    } else {
        CLIENT_WITHOUT_CACHE.get_or_init(|| {
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
    client
        .execute(request)
        .map_err(|error| EnvError::Fetch(error.to_string()))
        .and_then(|resp| {
            if !resp.status().is_success() {
                Either::Right(future::err(EnvError::Fetch(format!(
                    "Unexpected HTTP status code {}",
                    resp.status().as_u16(),
                ))))
            } else {
                Either::Left(
                    resp.bytes()
                        .map_err(|error| EnvError::Fetch(error.to_string())),
                )
            }
        })
        .and_then(|body| {
            let mut deserializer = Deserializer::from_slice(body.as_ref());
            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    let result = serde_path_to_error::deserialize::<_, OUT>(&mut deserializer);
                } else {
                    let result = OUT::deserialize(&mut deserializer);
                }
            };
            future::ready(result.map_err(|error| EnvError::Serde(error.to_string())))
        })
        .boxed_env()
}
