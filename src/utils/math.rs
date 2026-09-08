pub struct MathUtils;

impl MathUtils {
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
        val.max(min).min(max)
    }
}
