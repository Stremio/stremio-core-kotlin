use jni::objects::{AutoLocal, JObject};
use jni::JNIEnv;

pub trait ExceptionDescribeExt<T, E> {
    fn exception_describe(self, env: &JNIEnv) -> Result<T, E>;
}

impl<T> ExceptionDescribeExt<T, jni::errors::Error> for jni::errors::Result<T> {
    fn exception_describe(self, env: &JNIEnv) -> Self {
        if self.is_err() {
            let _ = env.exception_describe();
        };
        self
    }
}

pub trait JObjectExt<'a> {
    fn get_class_name<'b>(self, env: &'b JNIEnv<'a>) -> jni::errors::Result<String>;
    fn auto_local<'b>(self, env: &'b JNIEnv<'a>) -> AutoLocal<'a, 'b>;
}

impl<'a> JObjectExt<'a> for JObject<'a> {
    fn get_class_name<'b>(self, env: &'b JNIEnv<'a>) -> jni::errors::Result<String> {
        let class = env.get_object_class(self)?;
        let class = env.auto_local(class);
        let class_name = env
            .call_method(class.as_obj(), "getName", "()Ljava/lang/String;", &[])?
            .l()?;
        let class_name = env.auto_local(class_name);
        let class_name = env
            .get_string(class_name.as_obj().into())?
            .to_string_lossy()
            .into_owned();
        Ok(class_name)
    }
    #[inline]
    fn auto_local<'b>(self, env: &'b JNIEnv<'a>) -> AutoLocal<'a, 'b> {
        env.auto_local(self)
    }
}
