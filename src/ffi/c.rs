use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn stardew_view_create(width: i32, height: i32) -> *mut std::ffi::c_void {
    let _ = (width, height);
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn stardew_view_destroy(_handle: *mut std::ffi::c_void) {
}

#[no_mangle]
pub extern "C" fn stardew_view_render(_handle: *mut std::ffi::c_void) {
}

#[no_mangle]
pub extern "C" fn stardew_view_resize(_handle: *mut std::ffi::c_void, _width: i32, _height: i32) {
}

#[no_mangle]
pub extern "C" fn stardew_view_get_version() -> *const c_char {
    static VERSION: &[u8] = b"0.30\0";
    VERSION.as_ptr() as *const c_char
}
