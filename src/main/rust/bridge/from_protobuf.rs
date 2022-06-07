pub trait FromProtobuf<T> {
    fn from_protobuf(&self) -> T;
}
