pub trait ToProtobuf<T, A> {
    fn to_protobuf(&self, args: &A) -> T;
}
