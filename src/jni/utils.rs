use jni::objects::JString;
use jni::JNIEnv;
use std::ffi::CStr;

pub fn jstring_to_string(env: &JNIEnv, jstr: JString) -> String {
    let c_str = env.get_string(jstr).unwrap();
    c_str.to_str().unwrap().to_string()
}
