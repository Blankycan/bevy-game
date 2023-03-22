use bevy::prelude::*;
use std::f32::consts::PI;

use bevy_sprite3d::AtlasSprite3dComponent;

use super::components::Direction;
use super::components::*;

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
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

pub fn set_character_direction(
    mut animated_character: &mut AnimatedCharacter,
    direction: Direction,
    mut atlas_sprite: &mut AtlasSprite3dComponent,
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

pub fn turning_toward_camera(
    mut object_query: Query<(&TurnTowardCamera, &mut Transform)>,
    camera_query: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
    time: Res<Time>,
) {
    let camera = camera_query.single();
    let look_position = (camera.translation - camera.forward() * 10.0) * Vec3::new(1.0, 0.0, 1.0);

    for (should_turn, mut obj_transform) in &mut object_query {
        if !should_turn.0 {
            return;
        }

        let rotation = Transform::from_translation(obj_transform.translation)
            .looking_at(look_position, Vec3::Y)
            .rotation
            .mul_quat(Quat::from_rotation_y(PI));
        obj_transform.rotation = obj_transform
            .rotation
            .slerp(rotation, time.delta_seconds() * 10.0);
    }
}

pub fn update_character_direction(
    mut query: Query<(&mut AnimatedCharacter, &mut AtlasSprite3dComponent)>,
    camera_query: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
) {
    let camera = camera_query.single();
    let look_position = (camera.translation - camera.forward() * 10.0) * Vec3::new(1.0, 0.0, 1.0);

    for (mut animated_character, mut atlas_sprite) in &mut query {
        let direction = get_character_direction(&animated_character, look_position);
        if direction != animated_character.direction {
            set_character_direction(&mut animated_character, direction, &mut atlas_sprite);
        }
    }
}

pub fn animate_sprite_system(
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
