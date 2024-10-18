#[doc(inline)]
pub use crate::{
    bridge::{FromProtobuf, ToProtobuf},
    protobuf::{
        // Re-export all google Protobuf specific structs
        google::protobuf::*,
        // Re-export Protobuf core modules on the API's top level
        stremio::core as stremio_core_models,
    },
};

pub mod bridge;

/// Model implementations.
/// The bridge between Core and Protobuf generated core data structures
///
/// Includes ToProtobuf and FromProtobuf impls for fields of models and custom models themseves
pub mod model {
    pub use addons::*;

    mod addons;

    // Impls of ToProtobuf & FromProtobuf
    mod fields;
}

#[allow(clippy::all)]
/// The generated module before any export's take place in the API's top level
/// Made public because of the compatibility with stremio-core-android
pub mod protobuf;
