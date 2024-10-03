use futures::future;
use objc2::runtime::AnyObject;
use serde::{Deserialize, Serialize};

use objc2_foundation::{NSString, NSUserDefaults};
use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

pub struct Storage {}
//TODO: This implimentation probably have race condition. Proper implimentation needed
impl Storage {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {})
    }

    pub fn get<T: for<'de> Deserialize<'de> + Send + 'static>(
        &self,
        key: &str,
    ) -> TryEnvFuture<Option<T>> {
        let key = key.to_owned();
        Box::pin(future::lazy(move |_| {
            let nskey = &NSString::from_str(&key);

            let user_defaults = unsafe { &NSUserDefaults::standardUserDefaults() };
            let optional_value = unsafe { &NSUserDefaults::stringForKey(user_defaults, nskey) };

            Ok(match optional_value {
                Some(value) => {
                    let deserialized_value: T =
                        serde_json::from_str(&value.to_string()).map_err(EnvError::from)?; // Adjust error handling as needed
                    Some(deserialized_value)
                }
                None => None,
            })
        }))
        .boxed_env()
    }
    pub fn set<T: Serialize>(&self, key: &str, value: Option<&T>) -> TryEnvFuture<()> {
        if let Some(value) = value {
            let nskey = &NSString::from_str(&key);
            let user_defaults = unsafe { &NSUserDefaults::standardUserDefaults() };

            // Convert the value to a JSON string
            let nsvalue = match serde_json::to_string(value) {
                Ok(value) => NSString::from_str(&value),
                Err(error) => return future::err(EnvError::Serde(error.to_string())).boxed_env(),
            };
            let nsvalue_object: Option<&AnyObject> = Some(&nsvalue);
            unsafe { NSUserDefaults::setObject_forKey(&user_defaults, nsvalue_object, nskey) };
        }
        future::lazy(move |_| {
            Ok(()) // Return the serialized value
        })
        .boxed_env()
    }
}
