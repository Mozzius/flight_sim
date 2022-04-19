use bevy::{core::FixedTimestep, prelude::*};
mod utils;

const TIME_STEP: f32 = 1.0 / 60.0;
const INITIAL_PLANE_ALTITUDE: f32 = 200.0;
const INITIAL_PLANE_SPEED: f32 = 1.0;
const MINIMUM_SPEED: f32 = 1.0;
const MAXIMUM_SPEED: f32 = 50.0;

const CAMERA_X: f32 = 0.0;
const CAMERA_Y: f32 = 5.0;
const CAMERA_Z: f32 = 20.0;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
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
    // note that we have to include the `Scene0` label
    let f22_raptor = asset_server.load("f22-raptor/scene.gltf#Scene0");
    let cityscape = asset_server.load("cityscape/scene.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a Transform and GlobalTransform
    // and spawn our gltf as a scene under it
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, INITIAL_PLANE_ALTITUDE, 0.0)),
            GlobalTransform::identity(),
        ))
        .insert(Player {
            speed: INITIAL_PLANE_SPEED,
        })
        .with_children(|parent| {
            // parent.spawn_bundle(PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
            //     ..default()
            // });
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

    commands.spawn_scene(cityscape);

    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 0.0, 10.0),
        ..default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(
            CAMERA_X,
            INITIAL_PLANE_ALTITUDE + CAMERA_Y,
            CAMERA_Z,
        )),
        ..default()
    });
}

fn update_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    let mut roll = 0.0;

    let mut camera = camera_query.single_mut();
    let (mut player_transform, mut player) = query.single_mut();

    let forwards = player_transform.forward();

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        roll -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        roll += 1.0;
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
        if player.speed < MAXIMUM_SPEED {
            player.speed += 0.1;
        }
    }

    if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
        if player.speed > MINIMUM_SPEED {
            player.speed -= 0.1;
        }
    }

    yaw *= 0.01;
    pitch *= 0.03;
    roll *= 0.03;
    
    player_transform.rotate(Quat::from_rotation_x(pitch));
    player_transform.rotate(Quat::from_rotation_y(yaw));
    player_transform.rotate(Quat::from_rotation_z(roll));

    // player_transform.rotation.slerp(current_rotation * Vec3::Y, 0.5);

    player.speed -= forwards.y * 0.1;

    if player.speed < MINIMUM_SPEED {
        player.speed = MINIMUM_SPEED;
    }

    player_transform.translation += forwards * player.speed;

    if player_transform.translation.y < 0.0 {
        player_transform.translation.y = 0.0;
    }

    camera.translation = utils::lerp_vec3(camera.translation, &player_transform.translation, 0.1)
        + player_transform
            .rotation
            .mul_vec3(Vec3::new(CAMERA_X, 0.0, CAMERA_Z))
        + Vec3::new(0.0, CAMERA_Y, 0.0);

    // camera.translation.lerp(
    //     player_transform.translation + Vec3::new(CAMERA_X, CAMERA_Y, CAMERA_Z),
    //     1.0,
    // );

    camera.look_at(player_transform.translation + (forwards * 100.0), Vec3::Y);

    // camera.rotation = camera.rotation.slerp(
    //     player_transform
    //         .rotation
    //         .mul_quat(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
    //     0.1,
    // );
}
