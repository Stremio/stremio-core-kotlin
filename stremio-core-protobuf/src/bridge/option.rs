use crate::bridge::{FromProtobuf, ToProtobuf};

impl<T: FromProtobuf<U>, U> FromProtobuf<Option<U>> for Option<T> {
    fn from_protobuf(&self) -> Option<U> {
        self.as_ref().map(|item| item.from_protobuf())
    }
}

impl<T: ToProtobuf<U, A>, U, A> ToProtobuf<Option<U>, A> for Option<T> {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(&self, args: &A) -> Option<U> {
        self.as_ref().map(|item| item.to_protobuf::<E>(args))
    }
}
