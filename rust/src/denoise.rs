use jni::JNIEnv;
use jni::objects::{JClass, JFloatArray, JObject, JValue};
use jni::sys::{jfloat, jlong, jsize};
use nnnoiseless::DenoiseState;
use crate::denoise_container::DenoiseContainer;
use crate::util::exception::{JavaException, JavaExceptions};
use crate::util::into_exception::ErrIntoException;
use crate::util::pointer::{get_pointer_from_field, JavaPointers};

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_createNative(
    mut env: JNIEnv,
    _class: JClass
) -> jlong {
    match create_denoise_state() {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_processNative<'local>(
    mut env: JNIEnv<'local>,
    denoise: JObject<'local>,
    samples: JFloatArray<'local>
) -> JFloatArray<'local> {
    match denoise_process(&mut env, denoise, samples) {
        Ok(processed) => processed,
        Err(exception) => {
            env.throw_new_exception(exception);
            env.new_float_array(0).expect("") // todo: ???
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_closeNative<'local>(
    mut env: JNIEnv<'local>,
    denoise: JObject<'local>
) {
    match denoise_close(&mut env, denoise) {
        Ok(()) => (),
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}


fn create_denoise_state() -> Result<jlong, JavaException> {
    let denoise = DenoiseState::new();

    let denoise_container = DenoiseContainer {
        denoise,
        first: false
    };

    Ok(denoise_container.into_jlong_pointer())
}

unsafe fn get_denoise<'local>(
    env: &mut JNIEnv,
    denoise: &JObject
) -> Result<&'local mut DenoiseContainer, JavaException> {
    let pointer = get_pointer_from_field(env, denoise, "pointer".into())
        .err_into_denoise_exception("Failed to get a pointer from the java object".into())?;

    Ok(DenoiseContainer::from_jlong_pointer(pointer))
}

unsafe fn denoise_close<'local>(
    env: &mut JNIEnv<'local>,
    denoise: JObject<'local>
) -> Result<(), JavaException> {
    let pointer = get_pointer_from_field(env, &denoise, "pointer".into())
        .err_into_denoise_exception("Failed to get a pointer from the java object".into())?;

    let _container = Box::from_raw(pointer as *mut DenoiseContainer);
    env.set_field(&denoise, "pointer", "J", JValue::from(0 as jlong))
        .err_into_denoise_exception("Failed set reset pointer".into())?;

    Ok(())
}

unsafe fn denoise_process<'local>(
    env: &mut JNIEnv<'local>,
    denoise: JObject<'local>,
    samples: JFloatArray<'local>
) -> Result<JFloatArray<'local>, JavaException> {
    let container = get_denoise(env, &denoise)?;

    let samples_length = env.get_array_length(&samples)
        .err_into_denoise_exception("Failed to get samples array length".into())?
        as usize;

    let mut samples_vec = vec![0f64 as jfloat; samples_length];

    env.get_float_array_region(samples, 0, &mut samples_vec)
        .err_into_denoise_exception("Failed to copy samples to rust vec".into())?;

    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];

    for chunk in samples_vec.chunks_exact(DenoiseState::FRAME_SIZE) {
        container.denoise.process_frame(&mut out_buf[..], chunk);

        if !container.first {
            output.extend_from_slice(&out_buf[..]);
        }
        container.first = false;
    }

    let output_java = env.new_float_array(output.len() as jsize)
        .err_into_denoise_exception("Failed to create java float array".into())?;

    env.set_float_array_region(&output_java, 0, &output)
        .err_into_denoise_exception("Failed to copy float vec into java float array".into())?;

    Ok(output_java)
}
