use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;

pub mod animation;
mod camera;
pub mod character;
use crate::animation::AnimationPlugin;
use crate::camera::CameraPlugin;
use crate::character::PlayerPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0))]
    #[asset(texture_atlas(columns = 9, rows = 3))]
    #[asset(path = "Characters.png")]
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
        .add_startup_system(spawn_basic_scene)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(10.0))),
            material: materials.add(Color::rgb(0.4, 0.8, 0.4).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    // Light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Point Light"));
}
