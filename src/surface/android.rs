#[cfg(target_os = "android")]
use ndk::native_window::NativeWindow;

#[cfg(target_os = "android")]
pub struct AndroidSurface {
    pub window: NativeWindow,
    pub width: i32,
    pub height: i32,
}

#[cfg(not(target_os = "android"))]
pub struct AndroidSurface;
