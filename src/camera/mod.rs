use bevy::prelude::*;

mod components;
mod systems;

use components::*;
use systems::*;

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FollowCamera>()
            .add_startup_system(spawn_camera)
            .add_systems((camera_follow, camera_control).in_set(OnUpdate(GameState::Playing)));
    }
}
