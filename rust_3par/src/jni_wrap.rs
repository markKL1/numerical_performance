
use jni::JNIEnv;
use jni::objects::{JClass, JString, jdouble, jint};
use jni::sys::jstring;
use crate::mul_test;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_com_keylane_JniTest_mul(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass,
    input: jint,
) -> jdouble {
    eprintln!("Hello world from Rust (JNI)!");

    let (sum, cpu_time_used) = mul_test(n);

    eprintln!("Sum of product elements = {:.6}.", sum);
    eprintln!("Time taken = {:.6} second.", cpu_time_used);
    sum
}
