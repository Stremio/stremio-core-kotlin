pub trait FromProtobuf<T> {
    #[allow(clippy::wrong_self_convention)]
    fn from_protobuf(&self) -> T;
}
