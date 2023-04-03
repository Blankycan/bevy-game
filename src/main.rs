use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod systems;
use systems::*;

pub mod animation;
mod camera;
pub mod character;
pub mod component_sprite;
use crate::animation::AnimationPlugin;
use crate::camera::CameraPlugin;
use crate::character::PlayerPlugin;
use crate::component_sprite::ComponentSpritePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 20.0, tile_size_y = 28.0))]
    #[asset(texture_atlas(columns = 4, rows = 9))]
    #[asset(path = "Character.png")]
    character_sheet: Handle<TextureAtlas>,
}

fn main() {
    println!("Starting Bevy app..");
    App::new()
        // Game states
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
        .insert_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Inspector
        .add_plugin(WorldInspectorPlugin::new())
        // Other plugins
        .add_plugin(Sprite3dPlugin)
        // Our systems
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(ComponentSpritePlugin)
        .add_startup_system(spawn_basic_scene)
        .add_system(change_nishita)
        .run();
}

fn change_nishita(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        info!("Changed to Atmosphere Preset 1 (Sunset)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0., 1., -1.),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key2) {
        info!("Changed to Atmosphere Preset 2 (Noir Sunset)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0., 1., -1.),
            rayleigh_coefficient: Vec3::new(1e-5, 1e-5, 1e-5),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key3) {
        info!("Changed to Atmosphere Preset 3 (Magenta)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            rayleigh_coefficient: Vec3::new(2e-5, 1e-5, 2e-5),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key4) {
        info!("Changed to Atmosphere Preset 4 (Strong Mie)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            mie_coefficient: 5e-5,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key5) {
        info!("Changed to Atmosphere Preset 5 (Larger Scale)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            rayleigh_scale_height: 16e3,
            mie_scale_height: 2.4e3,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key6) {
        info!("Changed to Atmosphere Preset 6 (Weak Intensity)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_intensity: 11.0,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key7) {
        info!("Changed to Atmosphere Preset 7 (Half Radius)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            ray_origin: Vec3::new(0., 6372e3 / 2., 0.),
            planet_radius: 6371e3 / 2.,
            atmosphere_radius: 6471e3 / 2.,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key8) {
        info!("Changed to Atmosphere Preset 8 (Sideways World)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            ray_origin: Vec3::new(6372e3, 0., 0.),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key9) {
        info!("Changed to Atmosphere Preset 9 (Inverted Mie Direction)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            mie_direction: -0.758,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key0) {
        info!("Reset Atmosphere to Default");
        commands.remove_resource::<AtmosphereModel>();
    }
}
