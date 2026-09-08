use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use std::sync::Mutex;
use crate::StardewView;

lazy_static::lazy_static! {
    static ref VIEW: Mutex<Option<StardewView>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_init(
    env: JNIEnv,
    _class: JClass,
    surface: JObject,
    width: jint,
    height: jint,
) -> jlong {
    let surface = env.get_surface(surface).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(surface).unwrap() };
    let backend = crate::gpu::vulkan::VulkanBackend::new(surface, width as u32, height as u32);
    let view = StardewView::new(backend.device().clone(), backend.surface().clone(), backend.config());
    let mut guard = VIEW.lock().unwrap();
    *guard = Some(view);
    1
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_render(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    terminal_state: JObject,
) {
    let mut guard = VIEW.lock().unwrap();
    if let Some(view) = guard.as_mut() {
        // Convert terminal_state to alacritty_terminal::Term
        // view.render(&term);
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_present(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let mut guard = VIEW.lock().unwrap();
    if let Some(view) = guard.as_mut() {
        view.present();
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_view_StardewViewNative_resize(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    width: jint,
    height: jint,
) {
    let mut guard = VIEW.lock().unwrap();
    if let Some(view) = guard.as_mut() {
        view.resize(width as u32, height as u32);
    }
}
