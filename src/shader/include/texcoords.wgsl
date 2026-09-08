struct Globals {
    width: f32,
    height: f32,
    scale: f32,
}

fn screen_to_uv(position: vec2<f32>, globals: Globals) -> vec2<f32> {
    return vec2<f32>(
        position.x / globals.width,
        1.0 - position.y / globals.height,
    );
}

fn uv_to_screen(uv: vec2<f32>, globals: Globals) -> vec2<f32> {
    return vec2<f32>(
        uv.x * globals.width,
        (1.0 - uv.y) * globals.height,
    );
}