
use ::jni::JNIEnv;
use ::jni::objects::{JClass};
use ::jni::sys::{jint, jdouble};
use ::matmullib::mul_test;

#[no_mangle]
pub extern "system" fn Java_com_keylane_JniTest_mul(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jdouble {
    eprintln!("Hello world from Rust (JNI)!");

    let (sum, cpu_time_used) = mul_test(n as usize);

    eprintln!("Sum of product elements = {:.6}.", sum);
    eprintln!("Time taken = {:.6} second.", cpu_time_used);
    sum
}
