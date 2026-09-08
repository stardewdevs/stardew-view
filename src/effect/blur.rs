pub struct BlurEffect {
    pub radius: f32,
    pub intensity: f32,
}

impl BlurEffect {
    pub fn new(radius: f32, intensity: f32) -> Self {
        Self { radius, intensity }
    }

    pub fn apply(&self) -> Vec<f32> {
        let kernel_size = (self.radius * 2.0 + 1.0) as u32;
        let mut weights = Vec::with_capacity(kernel_size as usize);
        let sigma = self.radius / 3.0;
        let two_sigma_sq = 2.0 * sigma * sigma;

        for i in 0..kernel_size {
            let x = i as f32 - self.radius;
            let weight = (-x * x / two_sigma_sq).exp() * self.intensity;
            weights.push(weight);
        }

        let sum: f32 = weights.iter().sum();
        for w in weights.iter_mut() {
            *w /= sum;
        }

        weights
    }

    pub fn get_kernel_size(&self) -> u32 {
        (self.radius * 2.0 + 1.0) as u32
    }
}
