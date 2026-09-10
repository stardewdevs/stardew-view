#[cfg(feature = "jni")]
use jni::objects::JClass;
#[cfg(feature = "jni")]
use jni::sys::{jint, jlong};
#[cfg(feature = "jni")]
use jni::JNIEnv;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VIEW: Mutex<Option<crate::StardewView>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_init(
    _env: JNIEnv,
    _class: JClass,
    _width: jint,
    _height: jint,
) -> jlong {
    1
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_present(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
    let mut guard = VIEW.lock().unwrap();
    if let Some(view) = guard.as_mut() {
        view.present();
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_resize(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
    width: jint,
    height: jint,
) {
    let mut guard = VIEW.lock().unwrap();
    if let Some(view) = guard.as_mut() {
        view.resize(width as u32, height as u32);
    }
}
