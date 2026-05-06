pub const fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff = (b - a).rem_euclid(std::f32::consts::TAU);
    let shortest = if diff > std::f32::consts::PI {
        diff - std::f32::consts::TAU
    } else {
        diff
    };
    a + shortest * t
}
