//! This module contains all the bridge impls which take [`stremio_core`] types
//! and allow `FromProtobuf` and `ToProtobuf` for Protobuf generates structs and enums.
//!
//! # Bridge

// Re-export the FromProtobuf and ToProtobuf from stremio_core_protobuf
pub use stremio_core_protobuf::{FromProtobuf, ToProtobuf};

mod android_model_field;

mod to_jni_byte_array;
pub use to_jni_byte_array::*;
