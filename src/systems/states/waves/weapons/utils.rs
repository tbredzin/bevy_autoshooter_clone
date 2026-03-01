use bevy::prelude::{GlobalTransform, Vec3};

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

// Find nearest enemy within weapon range
pub fn get_nearest_enemy(
    position: &GlobalTransform,
    enemies: Vec<&GlobalTransform>,
    max_range: f32,
) -> Option<Vec3> {
    enemies
        .iter()
        .filter_map(|enemy_transform| {
            let enemy_pos = enemy_transform.translation();
            let distance = position.translation().distance(enemy_pos);
            if distance <= max_range {
                Some((enemy_pos, distance))
            } else {
                None
            }
        })
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(&dist2).unwrap())
        .map(|(pos, _)| pos)
}
