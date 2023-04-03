use bevy::prelude::*;

mod components;
mod systems;
use systems::*;

use crate::GameState;

pub struct ComponentSpritePlugin;

impl Plugin for ComponentSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (build_component_sprite, load_assets, setup)
                .chain()
                .in_schedule(OnEnter(GameState::Playing)),
        );
    }
}
