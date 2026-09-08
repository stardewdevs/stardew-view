use glam::{Vec2, Vec3, Vec4};

pub struct VectorUtils;

impl VectorUtils {
    pub fn vec2(x: f32, y: f32) -> Vec2 {
        Vec2::new(x, y)
    }

    pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }

    pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4::new(x, y, z, w)
    }

    pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        a.lerp(b, t)
    }

    pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a.lerp(b, t)
    }

    pub fn lerp_vec4(a: Vec4, b: Vec4, t: f32) -> Vec4 {
        a.lerp(b, t)
    }

    pub fn distance_vec2(a: Vec2, b: Vec2) -> f32 {
        a.distance(b)
    }

    pub fn distance_vec3(a: Vec3, b: Vec3) -> f32 {
        a.distance(b)
    }
}
