use bevy::{core::FixedTimestep, prelude::*};
mod utils;

const TIME_STEP: f32 = 1.0 / 60.0;
const INITIAL_PLANE_ALTITUDE: f32 = 1000.0;
const INITIAL_PLANE_SPEED: f32 = 0.0;

const MINIMUM_SPEED: f32 = 1.0;
const MAXIMUM_SPEED: f32 = 50.0;

const MINIMUM_THRUST: f32 = 0.0;
const MAXIMUM_THRUST: f32 = 80.0;

const CAMERA_X: f32 = 0.0;
const CAMERA_Y: f32 = 5.0;
const CAMERA_Z: f32 = 20.0;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Camera3d;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flight Sim".to_string(),
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(
            97.0 / 255.0,
            195.0 / 255.0,
            242.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_player),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let f22_raptor = asset_server.load("f22-raptor/scene.gltf#Scene0");
    let cityscape = asset_server.load("cityscape/scene.gltf#Scene0");

    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, INITIAL_PLANE_ALTITUDE, 0.0)),
            GlobalTransform::identity(),
        ))
        .insert(Player {
            speed: INITIAL_PLANE_SPEED,
        })
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

    commands
        .spawn_bundle((
            Transform::from_scale(Vec3::new(10.0, 10.0, 10.0)),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(cityscape);
        });

    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 0.0, 10.0),
        ..default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(
                CAMERA_X,
                INITIAL_PLANE_ALTITUDE + CAMERA_Y,
                CAMERA_Z,
            )),
            ..default()
        })
        .insert(Camera3d);
}

fn update_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    let mut roll = 0.0;

    let mut camera = camera_query.single_mut();
    let (mut player_transform, mut player) = query.single_mut();

    let forwards = player_transform.forward();

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
        yaw -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
        if player.speed < MAXIMUM_THRUST {
            player.speed += 1.0;
        }
    }

    if keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl) {
        if player.speed > MINIMUM_THRUST {
            player.speed -= 0.3;
        }
    }

    yaw *= 0.01;
    pitch *= 0.03;
    roll *= 0.03;

    // many thanks to rchar
    let player_x = player_transform.local_x();
    let player_y = player_transform.local_y();
    let player_z = player_transform.local_z();
    let rot = Quat::from_axis_angle(player_x, pitch)
        * Quat::from_axis_angle(player_y, yaw)
        * Quat::from_axis_angle(player_z, roll);
    player_transform.rotate(rot);
    // player_transform.rotation.slerp(current_rotation * Vec3::Y, 0.5);

    player.speed -= forwards.y * 0.2;

    if player.speed < MINIMUM_SPEED {
        player.speed += 1.0;
    } else if player.speed > MAXIMUM_SPEED {
        player.speed -= 0.5;
    }

    player_transform.translation += forwards * player.speed;

    if player_transform.translation.y < 0.0 {
        player_transform.translation.y = 0.0;
    }

    camera.translation = utils::lerp_vec3(camera.translation, &player_transform.translation, 0.2)
        + player_transform
            .rotation
            .mul_vec3(Vec3::new(CAMERA_X, 0.0, CAMERA_Z))
        + Vec3::new(0.0, CAMERA_Y, 0.0);

    camera.look_at(player_transform.translation + (forwards * 100.0), Vec3::Y);
}
