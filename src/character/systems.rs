use std::f32::consts::PI;
use std::f32::EPSILON;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_sprite3d::{AtlasSprite3d, Sprite3dParams};

use super::components::*;
use crate::animation::components::Direction;
use crate::animation::components::*;
use crate::animation::systems::*;
use crate::ImageAssets;

pub fn spawn_player(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.character_sheet.clone(),
                pixels_per_metre: 28.0,
                partial_alpha: true,
                unlit: false,
                index: 0,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(1.0, -0.15, 2.0),
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
                        (AnimationState::Idle, Direction::Right),
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
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [3].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 4).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 5).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 6).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 7).to_vec(),
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
                pixels_per_metre: 28.0,
                partial_alpha: true,
                unlit: false,
                index: 0,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(-2.0, -0.15, -1.3),
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
                            frames: [0].to_vec(),
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
                        (AnimationState::Idle, Direction::Up),
                        Animation {
                            frames: [2].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [3].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 4).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 5).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 6).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 7).to_vec(),
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
                pixels_per_metre: 28.0,
                partial_alpha: true,
                unlit: false,
                index: 0,
                pivot: Some(Vec2::new(0.5, 0.0)),
                transform: Transform::from_xyz(-0.8, -0.15, -1.6),
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
                            frames: [0].to_vec(),
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
                        (AnimationState::Idle, Direction::Up),
                        Animation {
                            frames: [2].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Idle, Direction::Left),
                        Animation {
                            frames: [3].to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Down),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 4).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Right),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 5).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Up),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 6).to_vec(),
                            ..default()
                        },
                    ),
                    (
                        (AnimationState::Walk, Direction::Left),
                        Animation {
                            frames: core::array::from_fn::<usize, 8, _>(|i| i * 4 + 7).to_vec(),
                            ..default()
                        },
                    ),
                ]),
                ..default()
            },
            ..default()
        });
}

pub fn control_player(
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
