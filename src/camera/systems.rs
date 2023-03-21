use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use super::components::*;
use crate::character::components::Player;

pub fn spawn_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands
        .spawn(Camera3dBundle {
            transform: camera_transform,
            ..default()
        })
        .insert(FollowCamera {
            offset: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(AtmosphereCamera::default())
        .insert(Name::new("Follow Camera"));
}

pub fn camera_follow(
    mut camera_query: Query<(&mut Transform, &FollowCamera), (With<FollowCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let (mut camera_transform, follow_camera) = camera_query.single_mut();

    let rot_hor = Quat::from_axis_angle(Vec3::Y, follow_camera.rotation_horizontal);
    let rot_ver = Quat::from_axis_angle(Vec3::X, follow_camera.rotation_vertical);
    let target_rotation = rot_hor * rot_ver;
    let target_position = target_rotation.mul_vec3(Vec3::Z * follow_camera.zoom)
        + player_transform.translation
        + follow_camera.offset;
    camera_transform.rotation = camera_transform.rotation.lerp(
        target_rotation,
        follow_camera.speed_transition * time.delta_seconds(),
    );
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        follow_camera.speed_transition * time.delta_seconds(),
    );
}

pub fn camera_control(
    mut camera_query: Query<&mut FollowCamera, With<FollowCamera>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut follow_camera = camera_query.single_mut();

    // Scroll
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                follow_camera.zoom -= ev.y * follow_camera.zoom_speed * time.delta_seconds();
                follow_camera.zoom = follow_camera
                    .zoom
                    .clamp(follow_camera.zoom_limit_min, follow_camera.zoom_limit_max);
            }
            MouseScrollUnit::Pixel => {
                follow_camera.zoom -= ev.y * follow_camera.zoom_speed * time.delta_seconds();
                follow_camera.zoom = follow_camera
                    .zoom
                    .clamp(follow_camera.zoom_limit_min, follow_camera.zoom_limit_max);
            }
        }
    }

    // Rotate
    if mouse.pressed(MouseButton::Middle)
        || (mouse.pressed(MouseButton::Right)
            && keyboard.any_pressed([KeyCode::LShift, KeyCode::RShift]))
    {
        for ev in motion_evr.iter() {
            follow_camera.rotation_horizontal -=
                ev.delta.x * follow_camera.rotation_horizontal_speed * time.delta_seconds();
            follow_camera.rotation_horizontal = follow_camera.rotation_horizontal.clamp(
                follow_camera.rotation_horizontal_limit_min,
                follow_camera.rotation_horizontal_limit_max,
            );
            follow_camera.rotation_vertical +=
                ev.delta.y * follow_camera.rotation_vertical_speed * time.delta_seconds();
            follow_camera.rotation_vertical = follow_camera.rotation_vertical.clamp(
                follow_camera.rotation_vertical_limit_min,
                follow_camera.rotation_vertical_limit_max,
            );
        }
    }
}
