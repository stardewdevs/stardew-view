use jni::objects::{JClass, JObject, JValue};
use jni::sys::jlong;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_nativeInit(
    mut env: JNIEnv,
    _class: JClass,
    width: i32,
    height: i32,
) -> jlong {
    let _ = (&mut env, width, height);
    0
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_nativeRender(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let _ = (&mut env, handle);
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_nativePresent(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let _ = (&mut env, handle);
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_nativeResize(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    width: i32,
    height: i32,
) {
    let _ = (&mut env, handle, width, height);
}
