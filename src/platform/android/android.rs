use ndk::native_window::NativeWindow;
use wgpu::Surface;

pub struct AndroidPlatform {
    pub window: NativeWindow,
    pub surface: Surface,
    pub width: i32,
    pub height: i32,
}

impl AndroidPlatform {
    pub fn new(window: NativeWindow, instance: &wgpu::Instance) -> Self {
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        let width = window.width();
        let height = window.height();
        Self { window, surface, width, height }
    }
}
