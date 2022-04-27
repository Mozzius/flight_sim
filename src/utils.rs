use bevy::prelude::*;

use super::AI;

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return (1.0 - t) * v0 + t * v1;
}

pub fn lerp_vec3(mut v0: Vec3, v1: Vec3, t: f32) -> Vec3 {
    v0.x = lerp(v0.x, v1.x, t);
    v0.y = lerp(v0.y, v1.y, t);
    v0.z = lerp(v0.z, v1.z, t);
    return v0;
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn find_closest_target(targets: Vec<(Entity, &Transform)>, position: Vec3) -> Option<Entity> {
    let mut closest_enemy = None;
    let mut closest_distance = std::f32::MAX;

    for (entity, transform) in targets.iter() {
        let distance = (transform.translation - position).length();
        if distance < closest_distance {
            closest_enemy = Some(entity.clone());
            closest_distance = distance;
        }
    }

    return closest_enemy;
}
