fn normalize(angle: f32) -> f32 {
    let mut normalized = angle % std::f32::consts::TAU;
    if normalized < 0.0 {
        normalized += std::f32::consts::TAU;
    }
    normalized
}

/// Clamps an angle to the nearest point within a circular range
pub fn clamp_angle_to_range(angle: f32, min: f32, max: f32) -> f32 {
    let angle = normalize(angle);
    let min = normalize(min);
    let max = normalize(max);

    // Handle wraparound case
    if max < min {
        // Range wraps around 0
        if angle >= min || angle <= max {
            angle
        } else {
            // Pick closest boundary
            let dist_to_min = (angle - min)
                .abs()
                .min((angle + std::f32::consts::TAU - min).abs());
            let dist_to_max = (angle - max)
                .abs()
                .min((angle + std::f32::consts::TAU - max).abs());
            if dist_to_min < dist_to_max { min } else { max }
        }
    } else {
        // Normal case
        angle.clamp(min, max)
    }
}
