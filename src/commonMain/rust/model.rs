#[cfg(feature = "kotlin")]
pub use model::*;

#[cfg(feature = "kotlin")]
// model is only available when the feature is enabled
// because of the `AndroidEnv` impl
mod model;
