use jni::objects::JObject;
use jni::JNIEnv;

pub struct JniCallbacks;

impl JniCallbacks {
    pub fn on_initialized(env: &JNIEnv, callback: JObject) {
        // Call Java callback
    }

    pub fn on_frame_ready(env: &JNIEnv, callback: JObject) {
        // Call Java callback
    }
}
