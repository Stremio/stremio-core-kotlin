use chrono::{DateTime, TimeDelta, TimeZone, Utc};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::google::protobuf::Timestamp;

impl FromProtobuf<DateTime<Utc>> for Timestamp {
    fn from_protobuf(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.seconds, self.nanos as u32).unwrap()
    }
}

impl ToProtobuf<Timestamp, ()> for DateTime<Utc> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> Timestamp {
        Timestamp {
            seconds: self.timestamp(),
            nanos: self.timestamp_subsec_nanos() as i32,
        }
    }
}

impl ToProtobuf<Timestamp, ()> for TimeDelta {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, _args: &()) -> Timestamp {
        Timestamp {
            seconds: self.num_seconds(),
            nanos: self.num_nanoseconds().map_or(0, |nanos| nanos as i32),
        }
    }
}
