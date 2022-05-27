#![allow(clippy::module_inception)]

mod bridge;
mod env;
mod model;

mod jni_ext;
mod stremio_core_android;


pub mod stremio {
    pub mod core {
        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/stremio.core.types.rs"));
        }
        // pub mod runtime {
        //     include!(concat!(env!("OUT_DIR"), "/stremio.core.runtime.rs"));
        // }
        // pub mod models {
        //     include!(concat!(env!("OUT_DIR"), "/stremio.core.models.rs"));
        // }
    }
}


