use jni::sys::jlong;
use nnnoiseless::DenoiseState;
use crate::util::pointer::JavaPointers;

pub struct DenoiseContainer {
    pub denoise: Box<DenoiseState<'static>>,
    pub first: bool
}

impl JavaPointers<DenoiseContainer> for DenoiseContainer {
    fn into_jlong_pointer(self) -> jlong {
        Box::into_raw(Box::new(self)) as jlong
    }
}
