use bevy::{core::FixedTimestep, prelude::*};

use super::utils;
use super::{Camera3d, Controls, Player};

const TIME_STEP: f32 = 1.0 / 60.0;
const INITIAL_PLANE_ALTITUDE: f32 = 1000.0;

const MINIMUM_THRUST: f32 = 0.0;
const MAXIMUM_THRUST: f32 = 80.0;

const CAMERA_X: f32 = 0.0;
const CAMERA_Y: f32 = 5.0;
const CAMERA_Z: f32 = 20.0;

const GRAVITY: f32 = -9.81 * TIME_STEP;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(controls_system)
                .with_system(plane_system.after(controls_system)),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let f22_raptor = asset_server.load("models/f22-raptor/scene.gltf#Scene0");

    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, INITIAL_PLANE_ALTITUDE, 0.0)),
            GlobalTransform::identity(),
        ))
        .insert(Player::default())
        .with_children(|parent| {
            // center of the plane is not at 0,0 so offset slightly
            parent
                .spawn_bundle((
                    Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))
                        .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(f22_raptor);
                });
        });
}

fn controls_system(keyboard_input: Res<Input<KeyCode>>, mut controls: ResMut<Controls>) {
    let mut pitch = 0.0;
    let mut roll = 0.0;
    let mut yaw = 0.0;
    let mut thrust = 0.0;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        roll += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        roll -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        pitch -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        pitch += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        yaw += 1.0;
    }
    if keyboard_input.pressed(KeyCode::E) {
        yaw -= 1.0
    }
    if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
        thrust += 1.0;
    }
    if keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl) {
        thrust -= 1.0;
    }

    controls.yaw = utils::lerp(controls.yaw, yaw, 0.1);
    controls.pitch = utils::lerp(controls.pitch, pitch, 0.1);
    controls.roll = utils::lerp(controls.roll, roll, 0.1);
    controls.thrust = utils::clamp(thrust, MINIMUM_THRUST, MAXIMUM_THRUST);
}

fn plane_system(
    controls: Res<Controls>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut camera = camera_query.single_mut();
    let (mut player_transform, mut player) = query.single_mut();

    let forwards = player_transform.forward();

    let axis_deviance = Vec3::normalize_or_zero(player.velocity)
        .dot(Vec3::normalize_or_zero(forwards))
        / forwards.length();
    let on_axis_speed = player.velocity.length() * axis_deviance;

    let yaw = controls.yaw * 0.01 * axis_deviance;
    let pitch = controls.pitch * 0.03 * axis_deviance;
    let roll = controls.roll * 0.05 * axis_deviance;

    // many thanks to rchar
    let player_x = player_transform.local_x();
    let player_y = player_transform.local_y();
    let player_z = player_transform.local_z();
    let rot = Quat::from_axis_angle(player_x, pitch)
        * Quat::from_axis_angle(player_y, yaw)
        * Quat::from_axis_angle(player_z, roll);
    player_transform.rotate(rot);

    // thrust
    player.velocity += forwards * (controls.thrust.powf(1.0 / 4.0) * 0.3);

    // nudge velocity vector towards the forwards vector
    player.velocity = utils::lerp_vec3(
        player.velocity,
        player_transform.forward() * player.velocity.length(),
        utils::clamp(axis_deviance, 0.1, 1.0) * 0.05,
    );

    // drag
    // should be done depending on axis
    player.velocity *= utils::clamp(axis_deviance, 0.99, 1.0);

    // gravity
    player.velocity += Vec3::new(0.0, GRAVITY, 0.0);

    // lift
    if on_axis_speed > 30.0 {
        player.velocity += Vec3::new(0.0, -GRAVITY, 0.0) * axis_deviance;
    } else {
        player.velocity += Vec3::new(0.0, -GRAVITY, 0.0) * axis_deviance * (on_axis_speed / 30.0);
    }

    player_transform.translation += player.velocity;

    if player_transform.translation.y < 0.0 {
        player_transform.translation.y = 0.0;
    }

    camera.translation = utils::lerp_vec3(camera.translation, player_transform.translation, 0.2)
        + player_transform
            .rotation
            .mul_vec3(Vec3::new(CAMERA_X, 0.0, CAMERA_Z))
        + Vec3::new(0.0, CAMERA_Y, 0.0);

    camera.look_at(player_transform.translation + (forwards * 100.0), Vec3::Y);

    // set player stalling for UI
    player.stalling = axis_deviance < 0.5;
}
