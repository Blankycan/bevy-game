use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};

use crate::{ImageAssets, GameState, GameSystemLabel};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<TurnTowardCamera>()
            // On enter
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player.label(GameSystemLabel::Player))
            )
            // On update
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(animate_sprite_system)
                    .with_system(turning_toward_camera)
                    .with_system(control_player)
            );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(AtlasSprite3d {
            atlas: images.character_sheet.clone(),
            pixels_per_metre: 16.,
            partial_alpha: true,
            unlit: true,
            index: 12,
            pivot: Some(Vec2::new(0.5, 0.0)),
            transform: Transform::from_xyz(1.0, 0.0, 2.0),

            ..default()
        }
        .bundle(&mut sprite_params))
        .insert(Name::new("2D Player"))
        .insert(Player)
        .insert(CharacterBundle {
            ..default()
        });
    /*
    commands
    .spawn(PbrBundle {
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
        })
        .insert(
            AtlasSprite3d {
                atlas: images.character_sheet.clone(),
                
                pixels_per_metre: 32.,
                partial_alpha: true,
                unlit: true,
                
                index: 3,
                
                ..default()
            }
            .bundle(&mut sprite_params),
        );
        */
}

fn control_player (
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = player_query.single_mut();
    let mut direction = Vec2::splat(0.0);

    if keyboard.pressed(KeyCode::W) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    
    transform.translation.x += direction.x * 1.0 * time.delta_seconds();
    transform.translation.z += direction.y * 1.0 * time.delta_seconds();
}

#[derive(Component)]
pub struct Player;


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TurnTowardCamera(pub bool);

pub fn turning_toward_camera(
    mut query: Query<(&TurnTowardCamera, &mut Transform)>,
    camera: Query<&Transform, (With<Camera>, Without<TurnTowardCamera>)>,
    time: Res<Time>,
) {
    let camera = camera.single();
    for (should_turn, mut obj_transform) in &mut query {
        if should_turn.0 == true {
            let mut look_position = camera.translation - obj_transform.translation;
            look_position.y = 0.0;
            let rotation = Transform::IDENTITY.looking_at(look_position, Vec3::Y).rotation;
            obj_transform.rotation = obj_transform.rotation.slerp(rotation, time.delta_seconds() * 10.0);
        }
    }
}

enum Direction {
    Down,
    Left,
    Up,
    Right
}

#[derive(Component)]
struct AnimatedCharacter {
    direction: Direction,

}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AtlasSprite3dComponent)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % sprite.atlas.len();
        }
    }
}

#[derive(Bundle)]
struct CharacterBundle {
    turn_to_camera: TurnTowardCamera,
    //sprite: AtlasSprite3d,
    animate_timer: AnimationTimer,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            turn_to_camera: TurnTowardCamera(true),
            animate_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}
