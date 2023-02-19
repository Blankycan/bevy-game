
use std::fmt;
use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Walk
}
impl fmt::Display for AnimationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "Idle"),
            AnimationState::Walk => write!(f, "Walk")
        }
    }
}

#[derive(Reflect, PartialEq)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub frame: usize,
    pub speed: f32
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            frame: 0,
            speed: 0.1
        }
    }
}
/*
impl Eq for Animation {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames &&
        self.frame == other.frame &&
        self.speed.partial_cmp(other.speed) == Some(Equal)
    }
}
*/