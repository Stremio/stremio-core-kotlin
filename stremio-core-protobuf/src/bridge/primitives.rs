use stremio_core::runtime::Env;

use super::{FromProtobuf, ToProtobuf};

impl ToProtobuf<i64> for u64 {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> i64 {
        i64::try_from(*self).unwrap_or_default()
    }
}

impl FromProtobuf<u64> for i64 {
    fn from_protobuf(&self) -> u64 {
        u64::try_from(*self).unwrap_or_default()
    }
}
