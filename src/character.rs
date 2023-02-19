use std::{
    f32::{consts::PI, EPSILON},
    fmt,
};

use bevy::{prelude::*, utils::HashMap};
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};

use crate::{animation::{AnimationState, Animation}, GameState, GameSystemLabel, ImageAssets};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            //.register_type::<AnimatedCharacter>()
            // On enter
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player.label(GameSystemLabel::Player))
                    .with_system(spawn_npcs),
            )
            // On update
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(animate_sprite_system)
                    .with_system(turning_toward_camera)
                    .with_system(control_player),
            );
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
        .insert(Name::new("2D Player"))
        .insert(Player)
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(1.0, 0.0, 0.0),
                animations: HashMap::from([
                    ((AnimationState::Idle, Direction::Down),  Animation { frames: [0].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Left),  Animation { frames: [1].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Up),    Animation { frames: [2].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Right), Animation { frames: [1].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Down),  Animation { frames: [3, 0, 6, 0].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Left),  Animation { frames: [4, 1, 7, 1].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Up),    Animation { frames: [5, 2, 8, 2].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Right), Animation { frames: [4, 1, 7, 1].to_vec(), ..default() }),
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
                    ((AnimationState::Idle, Direction::Down),  Animation { frames: [ 9].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Left),  Animation { frames: [10].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Up),    Animation { frames: [11].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Right), Animation { frames: [10].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Down),  Animation { frames: [12,  9, 15,  9].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Left),  Animation { frames: [13, 10, 16, 10].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Up),    Animation { frames: [14, 11, 17, 11].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Right), Animation { frames: [13, 10, 16, 10].to_vec(), ..default() }),
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
                    ((AnimationState::Idle, Direction::Down),  Animation { frames: [18].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Left),  Animation { frames: [19].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Up),    Animation { frames: [20].to_vec(), ..default() }),
                    ((AnimationState::Idle, Direction::Right), Animation { frames: [19].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Down),  Animation { frames: [21, 18, 24, 18].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Left),  Animation { frames: [22, 19, 25, 19].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Up),    Animation { frames: [23, 20, 26, 20].to_vec(), ..default() }),
                    ((AnimationState::Walk, Direction::Right), Animation { frames: [22, 19, 25, 19].to_vec(), ..default() }),
                ]),
                ..default()
            },
            ..default()
        });
}

fn control_player(
    mut player_query: Query<
        (&mut Transform, &mut AnimatedCharacter),
        (With<Player>, Without<Camera>),
    >,
    camera_query: Query<&Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut character_transform, mut animated_character) = player_query.single_mut();
    let mut direction = Vec3::splat(0.0);
    let mut speed = animated_character.walk_speed;

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
            // Try to reset the current animation frame before switching state
            let state = animated_character.animation_state;
            let direction = animated_character.direction;
            let animation = animated_character.animations.get_mut(&(state, direction));
            if animation.is_some() {
                animation.unwrap().frame = 0;
            }
            animated_character.animation_state = AnimationState::Idle;
        }
        return;
    }
    if keyboard.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        speed = animated_character.run_speed;
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
            // Try to reset the current animation frame before switching state
            let state = animated_character.animation_state;
            let direction = animated_character.direction;
            let animation = animated_character.animations.get_mut(&(state, direction));
            if animation.is_some() {
                animation.unwrap().frame = 0;
            }
            animated_character.animation_state = AnimationState::Walk;
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

#[derive(Component, Reflect, Default)]
pub struct TurnTowardCamera(pub bool);

fn turning_toward_camera(
    mut object_query: Query<(
        &TurnTowardCamera,
        &mut Transform,
        Option<&mut AnimatedCharacter>,
        Option<&mut AtlasSprite3dComponent>,
    )>,
    camera_query: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
    time: Res<Time>,
) {
    let camera = camera_query.single();
    for (should_turn, mut obj_transform, animated_character, atlas_sprite) in &mut object_query {
        if should_turn.0 {
            let mut look_position = camera.translation - obj_transform.translation;
            look_position.y = 0.0;
            let rotation = Transform::IDENTITY
                .looking_at(look_position, Vec3::Y)
                .rotation;
            obj_transform.rotation = obj_transform
                .rotation
                .slerp(rotation, time.delta_seconds() * 10.0);

            // Animated characters should be turned the correct way
            if let Some(mut animated_character) = animated_character {
                let direction = get_character_direction(&animated_character, look_position);
                if direction != animated_character.direction {
                    if let Some(mut atlas_sprite) = atlas_sprite {
                        set_character_direction(
                            &mut animated_character,
                            direction,
                            &mut atlas_sprite,
                            &mut obj_transform,
                        );
                    }
                }
            }
        }
    }
}

fn get_character_direction(
    animated_character: &AnimatedCharacter,
    viewing_position: Vec3,
) -> Direction {
    let towards_camera = animated_character.heading - viewing_position;
    let angle = animated_character.heading.angle_between(viewing_position);

    if angle < (PI / 4.0) {
        Direction::Down
    } else if angle > (3.0 * (PI / 4.0)) {
        Direction::Up
    } else {
        let right =
            Quat::from_euler(EulerRot::XYZ, 0.0, PI * 0.5, 0.0) * animated_character.heading;
        let dot = towards_camera.dot(right);
        if dot < 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    }
}

fn set_character_direction(
    mut animated_character: &mut AnimatedCharacter,
    direction: Direction,
    mut atlas_sprite: &mut AtlasSprite3dComponent,
    mut character_transform: &mut Transform,
) {
    
    let state = animated_character.animation_state;
    let prev_direction = animated_character.direction;
    let mut current_frame_index = 0 as usize;
    let mut new_sprite_index = atlas_sprite.index;
    
    // Get the previous animation
    {
        let prev_animation = animated_character.animations.get_mut(&(state, prev_direction));
        
        // Find which index in the animation we're currently on
        if prev_animation.is_some() {
            let mut anim = prev_animation.unwrap();
            current_frame_index = anim.frame;
            anim.frame = 0;
        }
    }

    // Get the new animation, and use the frame with the same index if possible
    {
        let new_animation = animated_character.animations.get_mut(&(state, direction));
        if new_animation.is_some() {
            let mut anim = new_animation.unwrap();
            if current_frame_index < anim.frames.len() {
                anim.frame = current_frame_index;
            }
            else {
                anim.frame = 0;
            }
            new_sprite_index = anim.frames[anim.frame];
        }
    }

    // All right-facing sprites are scaled with negative X
    if direction == Direction::Right {
        character_transform.scale.x = -(character_transform.scale.x).abs();
    } else {
        character_transform.scale.x = (character_transform.scale.x).abs();
    }

    atlas_sprite.index = new_sprite_index;
    animated_character.direction = direction;
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Down,
    Left,
    Up,
    Right,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(Component)]
struct AnimatedCharacter {
    // The orientation that the character is facing
    heading: Vec3,
    walk_speed: f32,
    run_speed: f32,
    // The direction the character is show, from camera's perspective
    direction: Direction,
    // What animation the character is performing
    animation_state: AnimationState,
    animations: HashMap<(AnimationState, Direction), Animation>
}

impl Default for AnimatedCharacter {
    fn default() -> Self {
        Self {
            heading: Vec3::Z,
            walk_speed: 2.0,
            run_speed: 5.0,
            direction: Direction::Down,
            animation_state: AnimationState::Idle,
            animations: HashMap::new()
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut AtlasSprite3dComponent,
        &mut AnimatedCharacter,
    )>,
) {
    for (mut timer, mut atlas_sprite, mut animated_character) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            // Get the correct animation
            let state = animated_character.animation_state;
            let direction = animated_character.direction;
            let animation = animated_character.animations.get_mut(&(state, direction));
            if animation.is_none() {
                return;
            }
            
            // Step forward one frame
            let mut anim = animation.unwrap();
            anim.frame += 1;
            if anim.frame >= anim.frames.len() {
                anim.frame = 0;
            }
            atlas_sprite.index = anim.frames[anim.frame];
        }
    }
}

#[derive(Bundle)]
struct CharacterBundle {
    turn_to_camera: TurnTowardCamera,
    animate_timer: AnimationTimer,
    animated_character: AnimatedCharacter,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            turn_to_camera: TurnTowardCamera(true),
            animate_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animated_character: AnimatedCharacter { ..default() },
        }
    }
}
