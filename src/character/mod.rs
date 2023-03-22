use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On enter
            .add_systems((spawn_player, spawn_npcs).in_schedule(OnEnter(GameState::Playing)))
            // On update
            .add_system(control_player.in_set(OnUpdate(GameState::Playing)));
    }
}
