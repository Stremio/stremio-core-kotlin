#![allow(clippy::module_inception)]

#[cfg(feature = "kotlin")]
// Re-export the kotlin-specific impls
pub use stremio_core_kotlin::*;

#[cfg(feature = "kotlin")]
pub mod bridge;

#[cfg(feature = "kotlin")]
/// Contains all kotlin (for android) related implementations for the bridge between
/// Rust and Kotlin.
///
/// - [AndroidEnv](crate::env::AndroidEnv)
/// - [Storage](crate::env::Storage)
/// - [Making requests](crate::env::fetch)
/// - [AndroidEvent](crate::env::AndroidEvent)s
pub mod env {
    mod env;
    pub use env::*;

    mod event;
    pub use event::*;

    mod fetch;
    pub use fetch::*;

    mod kotlin_class_name;
    pub use kotlin_class_name::*;

    mod storage;
    pub use storage::*;
}

pub mod model {
    #[cfg(feature = "kotlin")]
    pub use model::*;

    #[cfg(feature = "kotlin")]
    // model is only available when the feature is enabled
    // because of the `AndroidEnv` impl
    mod model;
}
#[allow(clippy::all)]
/// Protobuf generated module
pub mod protobuf {
    pub use stremio_core_protobuf::protobuf::*;
}

#[cfg(feature = "kotlin")]
pub mod jni_ext;
#[cfg(feature = "kotlin")]
mod stremio_core_kotlin;
