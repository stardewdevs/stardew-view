use glam::{Mat4, Vec3};

pub struct MatrixUtils;

impl MatrixUtils {
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        Mat4::perspective_rh_gl(fov_y, aspect, near, far)
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4::orthographic_rh_gl(left, right, bottom, top, near, far)
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
        Mat4::look_at_rh(eye, center, up)
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4::from_translation(Vec3::new(x, y, z))
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4::from_scale(Vec3::new(x, y, z))
    }

    pub fn rotation_z(angle: f32) -> Mat4 {
        Mat4::from_rotation_z(angle)
    }
}
