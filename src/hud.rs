use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::{Camera3d, Controls, Enemy, Player};

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct StallWarningText;

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct Reticule(Entity);

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(stall_warning_system)
            .add_system(speed_system)
            // .add_system(reticule_system)
            .add_system(fps_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/RobotoMono/RobotoMono-Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(FpsText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(10.0),
                    left: Val::Percent(20.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Throttle: ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(SpeedText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                position: Rect {
                    bottom: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::with_section(
                        "!! STALL WARNING !!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::RED,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),

                    ..default()
                })
                .insert(StallWarningText);
        });
}

fn fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("{:.0}", average);
            }
        }
    }
}

fn stall_warning_system(
    mut text_query: Query<&mut Style, With<StallWarningText>>,
    player_query: Query<&Player>,
) {
    let player = player_query.single();
    for mut text in text_query.iter_mut() {
        if player.stalling {
            text.display = Display::Flex;
        } else {
            text.display = Display::None;
        }
    }
}

fn speed_system(mut text_query: Query<&mut Text, With<SpeedText>>, controls: Res<Controls>) {
    for mut text in text_query.iter_mut() {
        text.sections[1].value = format!("{:.0}", controls.thrust);
    }
}

fn reticule_system(
    windows: Res<Windows>,
    mut reticule_query: Query<(&mut Style, &Reticule)>,
    targets_query: Query<&GlobalTransform, With<Enemy>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    images: Res<Assets<Image>>,
) {
    let (camera, camera_transform) = camera_query.single();
    for (mut style, reticule) in reticule_query.iter_mut() {
        if let Ok(transform) = targets_query.get(reticule.0) {
            match camera.world_to_screen(&windows, &images, camera_transform, transform.translation)
            {
                Some(screen_position) => {
                    style.display = Display::Flex;
                    style.position = Rect {
                        left: Val::Px(screen_position.x),
                        top: Val::Px(screen_position.y),
                        ..default()
                    };
                }
                None => {
                    style.display = Display::None;
                }
            }
        } else {
            warn!("Could not find reticule for target");
        }
    }
}
