//! This module contains all the bridge impls which take [`stremio_core`] types
//! and allow `FromProtobuf` and `ToProtobuf` for Protobuf generates structs and enums.
//!
//! # Bridge

mod action;
mod android_model_field;
mod auth_request;
mod date;
mod env_error;
mod event;
mod events;
mod extra_value;
mod library_item;
mod link;
mod list;
mod loadable;
mod manifest;
mod meta_preview;
mod option;
mod pair;
mod poster_shape;
mod profile;
mod resource_loadable;
mod resource_path;
mod resource_request;
mod stream;
mod string;
mod subtitle;

mod to_protobuf;
pub use to_protobuf::*;

mod from_protobuf;
pub use from_protobuf::*;

mod to_jni_byte_array;
pub use to_jni_byte_array::*;
