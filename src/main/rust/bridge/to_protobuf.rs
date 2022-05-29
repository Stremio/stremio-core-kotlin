use prost::Message;

pub trait ToProtobuf<T: Message, A> {
    fn to_protobuf(&self, args: &A) -> T;
}

pub trait ToProtobufAny<T, A> {
    fn to_protobuf(&self, args: &A) -> T;
}
