pub struct FpsCounter {
    frame_count: u32,
    last_time: std::time::Instant,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_time: std::time::Instant::now(),
        }
    }

    pub fn tick(&mut self) -> f32 {
        self.frame_count += 1;
        let elapsed = self.last_time.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            let fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_time = std::time::Instant::now();
            return fps;
        }
        0.0
    }
}
