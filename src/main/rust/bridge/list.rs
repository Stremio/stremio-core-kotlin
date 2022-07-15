use crate::bridge::{FromProtobuf, ToProtobuf};

impl<T: FromProtobuf<U>, U> FromProtobuf<Vec<U>> for Vec<T> {
    fn from_protobuf(&self) -> Vec<U> {
        self.iter().map(|item| item.from_protobuf()).collect()
    }
}

impl<T: ToProtobuf<U, A>, U, A> ToProtobuf<Vec<U>, A> for Vec<T> {
    fn to_protobuf(&self, args: &A) -> Vec<U> {
        self.iter().map(|item| item.to_protobuf(args)).collect()
    }
}
