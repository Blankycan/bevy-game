use std::{f32::consts::PI, fmt};

use bevy::prelude::*;
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};

use crate::{ImageAssets, GameState, GameSystemLabel};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            .register_type::<AnimatedCharacter>()
            // On enter
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player.label(GameSystemLabel::Player))
                    .with_system(spawn_npcs)
            )
            // On update
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    //.with_system(animate_sprite_system)
                    .with_system(turning_toward_camera)
                    .with_system(control_player)
            );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(AtlasSprite3d {
            atlas: images.character_sheet.clone(),
            pixels_per_metre: 16.,
            partial_alpha: true,
            unlit: true,
            index: 12,
            pivot: Some(Vec2::new(0.5, 0.0)),
            transform: Transform::from_xyz(1.0, 0.0, 2.0),

            ..default()
        }
        .bundle(&mut sprite_params))
        .insert(Name::new("2D Player"))
        .insert(Player)
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(1.0, 0.0, 0.0),
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
        .spawn(AtlasSprite3d {
            atlas: images.character_sheet.clone(),
            pixels_per_metre: 16.,
            partial_alpha: true,
            unlit: true,
            index: 12,
            pivot: Some(Vec2::new(0.5, 0.0)),
            transform: Transform::from_xyz(-2.0, 0.0, -1.3),
            
            ..default()
        }
        .bundle(&mut sprite_params))
        .insert(Name::new("Brown"))
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(0.8, 0.0, -0.2).normalize(),
                anim_down: [9, 12, 15],
                anim_left: [10, 13, 16],
                anim_up:   [11, 14, 17],
                ..default()
            },
            ..default()
        });
        
    commands
        .spawn(AtlasSprite3d {
            atlas: images.character_sheet.clone(),
            pixels_per_metre: 16.,
            partial_alpha: true,
            unlit: true,
            index: 12,
            pivot: Some(Vec2::new(0.5, 0.0)),
            transform: Transform::from_xyz(-0.8, 0.0, -1.6),
            
            ..default()
        }
        .bundle(&mut sprite_params))
        .insert(Name::new("Pink"))
        .insert(CharacterBundle {
            animated_character: AnimatedCharacter {
                heading: Vec3::new(-1.8, 0.0, 0.2).normalize(),
                anim_down: [18, 21, 24],
                anim_left: [19, 22, 25],
                anim_up:   [20, 23, 26],
                ..default()
            },
            ..default()
        });
}

fn control_player (
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = player_query.single_mut();
    let mut direction = Vec2::splat(0.0);

    if keyboard.pressed(KeyCode::W) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    
    transform.translation.x += direction.x * 1.0 * time.delta_seconds();
    transform.translation.z += direction.y * 1.0 * time.delta_seconds();
}

#[derive(Component)]
pub struct Player;


#[derive(Component, Reflect, Default)]
pub struct TurnTowardCamera(pub bool);

fn turning_toward_camera(
    
    mut object_query: Query<(&TurnTowardCamera, &mut Transform, Option<&mut AnimatedCharacter>, Option<&mut AtlasSprite3dComponent>)>,
    camera: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
    time: Res<Time>,
) {
    let camera = camera.single();
    for (should_turn, mut obj_transform, mut animated_character, mut atlas_sprite) in &mut object_query {
        if should_turn.0 == true {
            let mut look_position = camera.translation - obj_transform.translation;
            look_position.y = 0.0;
            let rotation = Transform::IDENTITY.looking_at(look_position, Vec3::Y).rotation;
            obj_transform.rotation = obj_transform.rotation.slerp(rotation, time.delta_seconds() * 10.0);

            // Animated characters should be turned the correct way
            if let Some(mut animated_character) = animated_character {
                let direction = get_character_direction(&animated_character, look_position);
                if direction != animated_character.direction {
                    if let Some(mut atlas_sprite) = atlas_sprite {
                        set_character_direction(&mut animated_character, direction, &mut atlas_sprite, &mut obj_transform);
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
    
    if angle < (PI / 3.0) {
        return Direction::Down
    }
    else if angle > (2.0 * (PI / 3.0)) {
        return Direction::Up;
    }
    else {
        let right = Quat::from_euler(EulerRot::XYZ, 0.0, PI * 0.5, 0.0) * animated_character.heading;
        let dot = towards_camera.dot(right);
        if dot < 0.0 {
            return Direction::Right
        }
        else {
            return Direction::Left;
        }
    }
}

fn set_character_direction(
    mut animated_character: &mut AnimatedCharacter,
    direction: Direction,
    mut atlas_sprite: &mut AtlasSprite3dComponent,
    mut obj_transform: &mut Transform
) {
    // Find which index in the animation we're currently on
    let current_sprite = atlas_sprite.index;
    let current_anim_index_option = match animated_character.direction {
        Direction::Down => animated_character.anim_down.iter().position(|&x| x == current_sprite),
        Direction::Left => animated_character.anim_left.iter().position(|&x| x == current_sprite),
        Direction::Up => animated_character.anim_up.iter().position(|&x| x == current_sprite),
        Direction::Right => animated_character.anim_left.iter().position(|&x| x == current_sprite),
    };

    // Get the same index, but for the new rotation
    let anim_index = current_anim_index_option.unwrap_or(0);
    let new_sprite_index = match direction {
        Direction::Down => animated_character.anim_down[anim_index],
        Direction::Left => animated_character.anim_left[anim_index],
        Direction::Up => animated_character.anim_up[anim_index],
        Direction::Right => animated_character.anim_left[anim_index],
    };

    if direction == Direction::Right {
        obj_transform.scale.x = -(obj_transform.scale.x).abs();
    }
    else {
        obj_transform.scale.x = (obj_transform.scale.x).abs();
    }

    atlas_sprite.index = new_sprite_index;
    animated_character.direction = direction;
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Down,
    Left,
    Up,
    Right
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right")
        }
    }
}

#[derive(Reflect, Clone, Copy, PartialEq)]
enum Animation {
    Idle,
    Walk
}
impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Animation::Idle => write!(f, "Idle"),
            Animation::Walk => write!(f, "Walk")
        }
    }
}

#[derive(Component, Reflect)]
struct AnimatedCharacter {
    heading: Vec3,
    direction: Direction,
    animation: Animation,
    anim_down: [usize; 3],
    anim_left: [usize; 3],
    anim_up: [usize; 3]
}

impl Default for AnimatedCharacter {
    fn default() -> Self {
        Self {
            heading: Vec3::Z,
            direction: Direction::Down,
            animation: Animation::Idle,
            anim_down: [0, 3, 6],
            anim_left: [1, 4, 7],
            anim_up: [2, 5, 8]
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AtlasSprite3dComponent)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % sprite.atlas.len();
        }
    }
}

#[derive(Bundle)]
struct CharacterBundle {
    turn_to_camera: TurnTowardCamera,
    animate_timer: AnimationTimer,
    animated_character: AnimatedCharacter
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            turn_to_camera: TurnTowardCamera(true),
            animate_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animated_character: AnimatedCharacter { ..default() }
        }
    }
}
