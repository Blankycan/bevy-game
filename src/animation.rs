use bevy::{prelude::*, utils::HashMap};
use bevy_sprite3d::AtlasSprite3dComponent;
use std::{
    f32::{consts::PI, EPSILON},
    fmt,
};

use crate::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            // On update
            .add_systems((turning_toward_camera, animate_sprite_system).chain().in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Walk,
}
impl fmt::Display for AnimationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "Idle"),
            AnimationState::Walk => write!(f, "Walk"),
        }
    }
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
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

#[derive(Reflect)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current: usize,
    pub speed: f32,
    pub timer: Timer,
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            current: 0,
            speed: 0.1,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

impl PartialEq for Animation {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames
            && self.current == other.current
            && (self.speed - other.speed) < EPSILON
            && self.timer.duration() == other.timer.duration()
    }
}

#[derive(Component)]
pub struct AnimatedCharacter {
    // The orientation that the character is facing
    pub heading: Vec3,
    // The direction the character is shown, from camera's perspective
    pub direction: Direction,
    // What animation the character is performing
    pub animation_state: AnimationState,
    pub animations: HashMap<(AnimationState, Direction), Animation>,
}

impl Default for AnimatedCharacter {
    fn default() -> Self {
        Self {
            heading: Vec3::Z,
            direction: Direction::Down,
            animation_state: AnimationState::Idle,
            animations: HashMap::new(),
        }
    }
}

pub fn get_character_direction(
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

pub fn set_character_direction(
    mut animated_character: &mut AnimatedCharacter,
    direction: Direction,
    mut atlas_sprite: &mut AtlasSprite3dComponent,
    mut character_transform: &mut Transform,
) {
    if animated_character.direction == direction {
        return;
    }

    let state = animated_character.animation_state;
    let prev_direction = animated_character.direction;
    let mut current_frame_index = 0 as usize;
    let mut new_sprite_index = atlas_sprite.index;

    // Get the previous animation
    {
        let prev_animation = animated_character
            .animations
            .get_mut(&(state, prev_direction));

        // Find which index in the animation we're currently on
        if prev_animation.is_some() {
            let mut animation = prev_animation.unwrap();
            current_frame_index = animation.current;
            // Reset the animation
            animation.current = 0;
            animation.timer.reset();
        }
    }

    // Get the new animation, and use the frame with the same index if possible
    {
        let new_animation = animated_character.animations.get_mut(&(state, direction));
        if new_animation.is_some() {
            let mut animation = new_animation.unwrap();
            if current_frame_index < animation.frames.len() {
                animation.current = current_frame_index;
            } else {
                animation.current = 0;
            }
            new_sprite_index = animation.frames[animation.current];
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

pub fn set_animation_state(
    animated_character: &mut AnimatedCharacter,
    animation_state: AnimationState,
) {
    if animated_character.animation_state == animation_state {
        return;
    }

    reset_animation(animated_character);
    animated_character.animation_state = animation_state;
}

// Try to reset the current animation
pub fn reset_animation(animated_character: &mut AnimatedCharacter) {
    let state = animated_character.animation_state;
    let direction = animated_character.direction;
    let some_animation = animated_character.animations.get_mut(&(state, direction));
    if some_animation.is_some() {
        let animation = some_animation.unwrap();
        animation.current = 0;
        animation.timer.reset();
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimatedCharacter, &mut AtlasSprite3dComponent)>,
) {
    for (mut animated_character, mut atlas_sprite) in query.iter_mut() {
        // Get the correct animation
        let state = animated_character.animation_state;
        let direction = animated_character.direction;
        let some_animation = animated_character.animations.get_mut(&(state, direction));
        if some_animation.is_none() {
            continue;
        }

        let mut animation = some_animation.unwrap();
        let mut current_sprite_index = animation.frames[animation.current];

        // Update animation timer
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // Step forward one frame
            animation.current += 1;
            if animation.current >= animation.frames.len() {
                animation.current = 0;
            }
            current_sprite_index = animation.frames[animation.current];
        }

        // Update the atlas sprite
        if atlas_sprite.index != current_sprite_index {
            atlas_sprite.index = current_sprite_index;
        }
    }
}

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
            let mut look_position = camera.translation - camera.forward() * 10.0;
            look_position.y = 0.0;
            let rotation = Transform::from_translation(obj_transform.translation)
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
