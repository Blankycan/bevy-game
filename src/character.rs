use std::f32::{consts::PI, EPSILON};

use bevy::{prelude::*, utils::HashMap};
use bevy_sprite3d::{AtlasSprite3d, Sprite3dParams};

use crate::{
    animation::{
        set_animation_state, AnimatedCharacter, Animation, AnimationState, Direction,
        TurnTowardCamera,
    },
    GameState, ImageAssets,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            //.register_type::<AnimatedCharacter>()
            // On enter
            .add_systems((spawn_player, spawn_npcs).in_schedule(OnEnter(GameState::Playing)))
            // On update
            .add_system(control_player.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn spawn_player(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.character_sheet.clone(),
                pixels_per_metre: 16.,
                partial_alpha: true,
                unlit: true,
                index: 12,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(1.0, 0.0, 2.0),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(1.0, 0.0, 0.0),
                animations: HashMap::from([
                    (
                        (AnimationState::Idle, Direction::Down),
                        Animation {
                            frames: [0].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [1].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Up),
                        Animation {
                            frames: [2].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Right),
                        Animation {
                            frames: [1].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: [3, 0, 6, 0].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: [4, 1, 7, 1].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: [5, 2, 8, 2].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: [4, 1, 7, 1].to_vec(),
                            ..default()
                        },
                    ),
                ]),
                ..default()
            },
            ..default()
        });
}

pub fn spawn_npcs(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.character_sheet.clone(),
                pixels_per_metre: 16.,
                partial_alpha: true,
                unlit: true,
                index: 12,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(-2.0, 0.0, -1.3),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(Name::new("Brown"))
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(0.8, 0.0, -0.2).normalize(),
                animations: HashMap::from([
                    (
                        (AnimationState::Idle, Direction::Down),
                        Animation {
                            frames: [9].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [10].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Up),
                        Animation {
                            frames: [11].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Right),
                        Animation {
                            frames: [10].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: [12, 9, 15, 9].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: [13, 10, 16, 10].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: [14, 11, 17, 11].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: [13, 10, 16, 10].to_vec(),
                            ..default()
                        },
                    ),
                ]),
                ..default()
            },
            ..default()
        });

    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.character_sheet.clone(),
                pixels_per_metre: 16.,
                partial_alpha: true,
                unlit: true,
                index: 12,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(-0.8, 0.0, -1.6),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(Name::new("Pink"))
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(-1.8, 0.0, 0.2).normalize(),
                animations: HashMap::from([
                    (
                        (AnimationState::Idle, Direction::Down),
                        Animation {
                            frames: [18].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [19].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Up),
                        Animation {
                            frames: [20].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Right),
                        Animation {
                            frames: [19].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: [21, 18, 24, 18].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: [22, 19, 25, 19].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: [23, 20, 26, 20].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: [22, 19, 25, 19].to_vec(),
                            ..default()
                        },
                    ),
                ]),
                ..default()
            },
            ..default()
        });
}

fn control_player(
    mut player_query: Query<
        (&mut Transform, &Movable, &mut AnimatedCharacter),
        (With<Player>, Without<Camera>),
    >,
    camera_query: Query<&Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut character_transform, movable, mut animated_character) = player_query.single_mut();
    let mut direction = Vec3::splat(0.0);
    let mut speed = movable.walk_speed;

    if keyboard.pressed(KeyCode::W) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    // No movement
    if direction.length_squared() < EPSILON {
        if animated_character.animation_state != AnimationState::Idle {
            set_animation_state(&mut animated_character, AnimationState::Idle);
        }
        return;
    }
    if keyboard.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        speed = movable.run_speed;
    }

    // Transform the vector based on the camera
    let camera_transform = camera_query.single();
    let (forward, right) = get_flat_camera_forward_and_right(camera_transform);
    let vertical = direction * forward;
    let horizontal = direction * right;
    let direction_vector =
        Vec3::new(horizontal.x + horizontal.z, 0.0, vertical.x + vertical.z).normalize();

    let move_force = direction_vector * speed * time.delta_seconds();
    move_character(
        &mut character_transform,
        &move_force,
        Some(&mut animated_character),
    );
}

fn move_character(
    character_transform: &mut Transform,
    move_force: &Vec3,
    animated_character_option: Option<&mut AnimatedCharacter>,
) {
    // Apply the heading and update the character direction if necessary
    if animated_character_option.is_some() {
        let mut animated_character = animated_character_option.unwrap();

        animated_character.heading = *move_force;
        if animated_character.animation_state != AnimationState::Walk {
            set_animation_state(&mut animated_character, AnimationState::Walk);
        }
    }

    character_transform.translation += *move_force;
}

// This removes the Y-component of the vectors, so the directions are flat to the ground.
fn get_flat_camera_forward_and_right(camera_transform: &Transform) -> (Vec3, Vec3) {
    let mut forward = camera_transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();
    let right = Quat::from_euler(EulerRot::XYZ, 0.0, -PI * 0.5, 0.0) * forward;

    (forward, right)
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable {
    walk_speed: f32,
    run_speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            walk_speed: 2.0,
            run_speed: 5.0,
        }
    }
}

#[derive(Bundle)]
struct CharacterBundle {
    movable: Movable,
    turn_to_camera: TurnTowardCamera,
    animated_character: AnimatedCharacter,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            movable: Movable { ..default() },
            turn_to_camera: TurnTowardCamera(true),
            animated_character: AnimatedCharacter { ..default() },
        }
    }
}
