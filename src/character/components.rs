use bevy::prelude::*;

use crate::animation::components::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable {
    pub walk_speed: f32,
    pub run_speed: f32,
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
pub struct CharacterBundle {
    pub movable: Movable,
    pub turn_to_camera: TurnTowardCamera,
    pub animated_character: AnimatedCharacter,
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
