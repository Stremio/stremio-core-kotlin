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

// pub use stremio_core_protobuf::{FromProtobuf, ToProtobuf};

pub trait ToProtobuf<T, A> {
    fn to_protobuf(&self, args: &A) -> T;
}

pub trait FromProtobuf<T> {
    #[allow(clippy::wrong_self_convention)]
    fn from_protobuf(&self) -> T;
}

mod to_jni_byte_array;
pub use to_jni_byte_array::*;
