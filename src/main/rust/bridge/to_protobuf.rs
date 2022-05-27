pub trait ToProtobuf<T> {
    fn to_protobuf(&self, args: &T) -> Vec<u8>;
}