use std::f32::{NEG_INFINITY, INFINITY};

use bevy::{prelude::*, input::mouse::{MouseWheel, MouseMotion}};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod character;
use crate::character::{PlayerPlugin, Player};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    Loading,
    Playing,
}

#[derive(SystemLabel)]
pub enum GameSystemLabel {
    Player,
    Camera
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
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .with_collection::<ImageAssets>(),
        )
        .add_state(GameState::Loading)
        .insert_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Inspector
        .add_plugin(WorldInspectorPlugin)
        // Other plugins
        .add_plugin(Sprite3dPlugin)
        // Our systems
        .add_startup_system(spawn_camera)
        .register_type::<FollowCamera>()
        .add_startup_system(spawn_basic_scene)
        .add_plugin(PlayerPlugin)
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(camera_follow)
                .with_system(camera_control)
        )
        .run();
}

#[derive(Component, Reflect)]
struct FollowCamera {
    // Offset to the center point of the target
    offset: Vec3,
    // Distance towards target
    zoom: f32,
    zoom_speed: f32,
    zoom_limit_min: f32,
    zoom_limit_max: f32,
    // Used to smooth the movement and rotation
    speed_transition: f32,
    // Used to rotate around the followed object
    rotation_horizontal: f32,
    rotation_horizontal_speed: f32,
    rotation_horizontal_limit_min: f32,
    rotation_horizontal_limit_max: f32,
    // Used to rotate up and down around the followed object
    rotation_vertical: f32,
    rotation_vertical_speed: f32,
    rotation_vertical_limit_min: f32,
    rotation_vertical_limit_max: f32
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            zoom: 8.0,
            zoom_speed: 30.0,
            zoom_limit_min: 2.0,
            zoom_limit_max: 18.0,
            speed_transition: 10.0,
            rotation_horizontal: 0.2,
            rotation_horizontal_speed: 0.5,
            rotation_horizontal_limit_min: NEG_INFINITY,
            rotation_horizontal_limit_max: INFINITY,
            rotation_vertical: -0.6,
            rotation_vertical_speed: 0.3,
            rotation_vertical_limit_min: -1.2,
            rotation_vertical_limit_max: -0.05
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..default()
    })
    .insert(FollowCamera {
        offset: Vec3::new(0.0, 0.32, 0.0),
        ..default()
    })
    .insert(Name::new("Follow Camera"));
}

fn camera_follow(
    mut camera_query: Query<(&mut Transform, &FollowCamera), (With<FollowCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>
) {
    let player_transform = player_query.single();
    let (mut camera_transform, follow_camera) = camera_query.single_mut();

    let rot_hor = Quat::from_axis_angle(Vec3::Y, follow_camera.rotation_horizontal);
    let rot_ver = Quat::from_axis_angle(Vec3::X, follow_camera.rotation_vertical);
    let target_rotation = rot_hor * rot_ver;
    let target_position = target_rotation.mul_vec3(Vec3::Z * follow_camera.zoom) + player_transform.translation + follow_camera.offset;
    camera_transform.rotation = camera_transform.rotation.lerp(target_rotation, follow_camera.speed_transition * time.delta_seconds());
    camera_transform.translation = camera_transform.translation.lerp(target_position, follow_camera.speed_transition * time.delta_seconds());
}

fn camera_control(
    mut camera_query: Query<&mut FollowCamera, With<FollowCamera>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut follow_camera = camera_query.single_mut();

    // Scroll
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                follow_camera.zoom -= ev.y * follow_camera.zoom_speed * time.delta_seconds();
                follow_camera.zoom = follow_camera.zoom.clamp(follow_camera.zoom_limit_min, follow_camera.zoom_limit_max);
            }
            MouseScrollUnit::Pixel => {
                follow_camera.zoom -= ev.y * follow_camera.zoom_speed * time.delta_seconds();
                follow_camera.zoom = follow_camera.zoom.clamp(follow_camera.zoom_limit_min, follow_camera.zoom_limit_max);
            }
        }
    }

    // Rotate
    if mouse.pressed(MouseButton::Middle) || (mouse.pressed(MouseButton::Right) && keyboard.any_pressed([KeyCode::LShift, KeyCode::RShift])) {
        for ev in motion_evr.iter() {
            follow_camera.rotation_horizontal -= ev.delta.x * follow_camera.rotation_horizontal_speed * time.delta_seconds();
            follow_camera.rotation_horizontal = follow_camera.rotation_horizontal.clamp(follow_camera.rotation_horizontal_limit_min, follow_camera.rotation_horizontal_limit_max);
            follow_camera.rotation_vertical += ev.delta.y * follow_camera.rotation_vertical_speed * time.delta_seconds();
            follow_camera.rotation_vertical = follow_camera.rotation_vertical.clamp(follow_camera.rotation_vertical_limit_min, follow_camera.rotation_vertical_limit_max);
        }
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
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
