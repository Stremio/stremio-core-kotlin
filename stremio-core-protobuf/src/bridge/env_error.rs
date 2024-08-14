use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::runtime;
use stremio_core::runtime::EnvError;

impl ToProtobuf<runtime::EnvError, ()> for EnvError {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> runtime::EnvError {
        runtime::EnvError {
            code: self.code() as i32,
            message: self.message(),
        }
    }
}
