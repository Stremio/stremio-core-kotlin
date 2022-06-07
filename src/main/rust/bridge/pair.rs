use crate::bridge::ToProtobuf;
use crate::protobuf::stremio::core::runtime;

impl ToProtobuf<runtime::PlanPair, ()> for (Vec<String>, Vec<String>) {
    fn to_protobuf(&self, _args: &()) -> runtime::PlanPair {
        runtime::PlanPair {
            first: self.0.clone(),
            second: self.1.clone(),
        }
    }
}
