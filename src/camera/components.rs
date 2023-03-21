use std::f32::{INFINITY, NEG_INFINITY};

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct FollowCamera {
    // Offset to the center point of the target
    pub offset: Vec3,
    // Distance towards target
    pub zoom: f32,
    pub zoom_speed: f32,
    pub zoom_limit_min: f32,
    pub zoom_limit_max: f32,
    // Used to smooth the movement and rotation
    pub speed_transition: f32,
    // Used to rotate around the followed object
    pub rotation_horizontal: f32,
    pub rotation_horizontal_speed: f32,
    pub rotation_horizontal_limit_min: f32,
    pub rotation_horizontal_limit_max: f32,
    // Used to rotate up and down around the followed object
    pub rotation_vertical: f32,
    pub rotation_vertical_speed: f32,
    pub rotation_vertical_limit_min: f32,
    pub rotation_vertical_limit_max: f32,
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
            rotation_vertical_limit_max: -0.05,
        }
    }
}
