use bevy::math::Vec3;

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return (1.0 - t) * v0 + t * v1;
}

pub fn lerp_vec3(mut v0: Vec3, v1: &Vec3, t: f32) -> Vec3 {
    v0.x = lerp(v0.x, v1.x, t);
    v0.y = lerp(v0.y, v1.y, t);
    v0.z = lerp(v0.z, v1.z, t);
    return v0;
}
