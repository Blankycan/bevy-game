use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

use crate::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            .register_type::<AnimatedCharacter>()
            // On update
            .add_systems(
                (turning_toward_camera, animate_sprite_system)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
