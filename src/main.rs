use bevy::prelude::*;
use bevy_atmosphere::*;

mod ai;
mod hud;
mod plane;
mod utils;

const INITIAL_PLANE_SPEED: f32 = 40.0;

#[derive(Component)]
pub struct Controls {
    pitch: f32,
    yaw: f32,
    roll: f32,
    thrust: f32,
    airbrakes: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            thrust: 50.0,
            airbrakes: false,
        }
    }
}

#[derive(Component)]
pub struct Player {
    velocity: Vec3,
    stalling: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity: Vec3::new(0.0, 0.0, -INITIAL_PLANE_SPEED),
            stalling: false,
        }
    }
}

#[derive(Component)]
pub struct AI {
    health: f32,
    velocity: Vec3,
    target: Option<Entity>,
}

impl Default for AI {
    fn default() -> Self {
        AI {
            health: 100.0,
            velocity: Vec3::new(0.0, 0.0, -1.0),
            target: None,
        }
    }
}

#[derive(Component)]
pub struct Ally;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Camera3d;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flight Sim".to_string(),
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AtmosphereMat {
            sun_position: Vec3::new(4.0, 10.0, 10.0),
            ..default()
        })
        .insert_resource(Controls::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin {
            dynamic: false,
            sky_radius: 1000000.0,
        })
        .add_plugin(hud::HUDPlugin)
        .add_plugin(plane::PlanePlugin)
        .add_plugin(ai::AIPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up the background
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cityscape = asset_server.load("models/cityscape/scene.gltf#Scene0");

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
        transform: Transform::from_xyz(4.0, 10.0, 10.0),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(Camera3d);
}
