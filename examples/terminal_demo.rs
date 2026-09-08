use std::time::{Duration, Instant};

use stardew_view::color::{Color, Palette};
use stardew_view::renderer::Renderer;
use stardew_view::utils::{FpsCounter, Timer};

fn main() {
    let palette = Palette {
        foreground: Color::GREEN,
        background: Color::BLACK,
        cursor: Color::GREEN,
        selection: Color::new(0.15, 0.31, 0.47, 1.0),
    };

    let mut fps = FpsCounter::new();
    let mut timer = Timer::new();
    let mut renderer = Renderer::default();

    let frame_count = 600;
    for frame in 0..frame_count {
        timer.reset();
        renderer.tick();
        fps.tick();
        println!(
            "frame {}: fps={:.1} frame_time={:?}",
            frame,
            fps.last_fps(),
            timer.elapsed()
        );
        if frame >= 30 {
            break;
        }
    }
    let _elapsed: Duration = timer.elapsed();
    let _ = palette;
}
