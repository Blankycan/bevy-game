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
) {
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
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(1.0, 1.0, 2.0),
        ..default()
    })
    .insert(Name::new("2D Player"))
    .insert(CharacterBundle {
        turn_to_camera: TurnTowardCamera(true),
        ..default()
    });

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

fn main() {
    println!("Starting Bevy app..");
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins)
        // Inspector
        .add_plugin(WorldInspectorPlugin)
        .register_type::<TurnTowardCamera>()
        // Our systems
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(turning_toward_camera)
        .run();
}
