use std::sync::Arc;

use block2::RcBlock;
use futures::future;
use http::{Method, Request};
use objc2::{rc::Retained, ClassType};
use objc2_foundation::{
    ns_string, NSData, NSError, NSHTTPURLResponse, NSMutableURLRequest, NSObjectProtocol, NSString,
    NSURLResponse, NSURLSession, NSURL,
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

use crate::stremio_core_apple::DEVICE_NAME;

/// User Agent used to make all calls to fetch.
///
/// This need to be initialized once on the first call to [`fetch`]
/// due to the device name being provided by the application on
/// initialization.
pub(crate) static USER_AGENT: OnceCell<Retained<NSString>> = OnceCell::new();

pub fn fetch<IN: Serialize + Send + 'static, OUT: for<'de> Deserialize<'de> + Send + 'static>(
    request: Request<IN>,
) -> TryEnvFuture<OUT> {
    // Initialize the OnceCell on the first call.
    // we expect the Device name to be initialized first by passing it from the application
    let user_agent = USER_AGENT.get_or_init(|| {
        let package_version = env!("CARGO_PKG_VERSION");
        let device_name = DEVICE_NAME.get().cloned().unwrap_or("Unknown".to_string());
        NSString::from_str(format!("Stremio-Apple/{package_version} {device_name}").as_str())
    });

    let (parts, body) = request.into_parts();

    let mut request = unsafe {
        let url =
            NSURL::initWithString(NSURL::alloc(), &NSString::from_str(&parts.uri.to_string()));
        NSMutableURLRequest::initWithURL(NSMutableURLRequest::alloc(), &url.unwrap())
    };
    unsafe { request.setValue_forHTTPHeaderField(Some(user_agent), ns_string!("User-Agent")) };

    match serde_json::to_string(&body) {
        Ok(body) if body != "null" && parts.method != Method::GET => {
            let nsbody = NSData::with_bytes(body.as_bytes());
            unsafe {
                request.setHTTPMethod(ns_string!("POST"));
                request.setHTTPBody(Some(&nsbody));
            }
            nsbody
        }
        Ok(_) => unsafe { NSData::data() },
        Err(error) => return future::err(EnvError::Serde(error.to_string())).boxed_env(),
    };

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Result<Vec<u8>, EnvError>>(1);
    let sender = Arc::new(sender);

    let completion_handler = RcBlock::new(
        enclose::enclose!((sender) move |data: *mut NSData, response: *mut NSURLResponse, error: *mut NSError| {
            let result = if !error.is_null() {
                let err = unsafe { &*error }.retain();

                Err(EnvError::Fetch(err.localizedDescription().to_string()))
            } else {
                let data: Option<&[u8]> = {
                    if !data.is_null() {
                        Some(unsafe { &*data }.bytes())
                    } else{
                        None
                    }
                };

                let response: Option<Retained<NSHTTPURLResponse>> = {
                    if !response.is_null() {
                        let response = unsafe { &*response };
                        if response.isKindOfClass(NSHTTPURLResponse::class()){
                            Some(unsafe { &*(response as *const NSURLResponse as *const NSHTTPURLResponse) }.retain())
                        }else{
                            None
                        }
                    } else{
                        None
                    }
                };

                match (data, response) {
                    (data, Some(response)) => {
                        let resp_code = unsafe { response.statusCode() };
                        if !(resp_code >= 200 && resp_code < 300 ) {
                            Err(EnvError::Fetch(format!(
                                "Unexpected HTTP status code {}",
                                resp_code,
                            )))
                        } else {
                            data.map(|buf| buf.to_vec()).ok_or(EnvError::Fetch("Response data is missing".into()))
                        }
                    },
                    _ => Err(EnvError::Fetch("Failed to fetch any response from the request".into()))
                }
            };


            // TODO: this is very tricky, somehow we need to pass the information
            // back to the receiver, but guard against null pointers, e.g.
            // we have data but not an Error

            if let Err(err) = futures::executor::block_on(sender.send(result)) {
                eprintln!("Failed to send values: {err}");
            }
        }),
    );

    unsafe {
        let nsurlsession = NSURLSession::sharedSession();
        let task = NSURLSession::dataTaskWithRequest_completionHandler(
            &nsurlsession,
            &request.as_super(),
            &completion_handler,
        );
        task.resume();
    }

    async move {
        let receive_result = receiver.recv().await.ok_or(EnvError::Other(
            "Channel closed, should never happen".to_string(),
        ))?;

        receive_result.and_then(|data| {
            let mut deserializer = Deserializer::from_slice(&data);
            let result = serde_path_to_error::deserialize::<_, OUT>(&mut deserializer);

            result.map_err(|error| EnvError::Serde(error.to_string()))
        })
    }
    .boxed_env()
}
