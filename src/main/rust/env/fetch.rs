use futures::future::Either;
use futures::{future, TryFutureExt};
use http::{Method, Request};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::convert::TryFrom;
use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

pub fn fetch<IN: Serialize, OUT: for<'de> Deserialize<'de> + Send + 'static>(
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
    Client::new()
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
