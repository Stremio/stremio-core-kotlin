use crate::jni_ext::ExceptionDescribeExt;
use jni::sys::jbyteArray;
use jni::JNIEnv;

pub trait ToJNIByteArray {
    fn to_jni_byte_array(&self, env: &JNIEnv) -> jbyteArray;
}

impl ToJNIByteArray for Vec<u8> {
    fn to_jni_byte_array(&self, env: &JNIEnv) -> jbyteArray {
        env.byte_array_from_slice(self)
            .exception_describe(env)
            .expect("protobuf conversion failed")
    }
}
