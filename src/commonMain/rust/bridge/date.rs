use chrono::{DateTime, TimeZone, Utc};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::google::protobuf::Timestamp;

impl FromProtobuf<DateTime<Utc>> for Timestamp {
    fn from_protobuf(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.seconds, self.nanos as u32).unwrap()
    }
}

impl ToProtobuf<Timestamp, ()> for DateTime<Utc> {
    fn to_protobuf(&self, _args: &()) -> Timestamp {
        Timestamp {
            seconds: self.timestamp(),
            nanos: self.timestamp_subsec_nanos() as i32,
        }
    }
}
