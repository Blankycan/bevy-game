use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod character;
use character::{CharacterBundle, TurnTowardCamera, turning_toward_camera};



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // Check out https://github.com/FraserLee/bevy_sprite3d
    let texture_handle = asset_server.load("Characters.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 22.0), 8, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Texture material for player
    let material_handle = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.4, 0.8, 0.4).into()),
        ..default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5,
            ..default()
        })),
        material: materials.add(Color::rgb(0.4, 0.45, 0.8).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    })
    .insert(Name::new("Capsule"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(1.0, 2.0),
            ..default()
        })),
        material: material_handle,
        transform: Transform::from_xyz(1.0, 1.0, 2.0),
        ..default()
    })
    .insert(Name::new("2D Player"))
    .insert(CharacterBundle {
        turn_to_camera: TurnTowardCamera(true),
        ..default()
    })
    .insert(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    })
    .insert(AnimationTimer(
        Timer::from_seconds(0.1, TimerMode::Repeating)
    ));

    // Light
    commands.spawn(PointLightBundle {
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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    )>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn main() {
    println!("Starting Bevy app..");
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Inspector
        .add_plugin(WorldInspectorPlugin)
        .register_type::<TurnTowardCamera>()
        // Our systems
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(turning_toward_camera)
        .add_system(animate_sprite)
        .run();
}
