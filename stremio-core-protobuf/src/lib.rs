#[allow(clippy::all)]
/// The generated module before any export's take place in the API's top level
/// Made public because of the compatibility with stremio-core-android
pub mod protobuf;

#[doc(inline)]
pub use protobuf::{
    // Re-export all google protobuf specific structs
    google::protobuf::*,
    // Re-export all core modules on the API's top level
    stremio::core::*,
};
