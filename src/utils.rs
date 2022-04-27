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

pub fn get_rotation_between(u: Vec3, v: Vec3) -> Quat {
    let add = u.normalize() + v.normalize();
    let angle = add.dot(v);
    let mut axis = add.cross(v);
    if axis.length() < 0.00001 {
        axis = Vec3::Z;
    } else {
        axis = axis.normalize()
    }
    return Quat::from_axis_angle(axis, angle);
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
