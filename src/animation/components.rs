use bevy::prelude::*;
use bevy::utils::HashMap;
use core::fmt;
use std::f32::EPSILON;

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Walk,
}
impl fmt::Display for AnimationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "Idle"),
            AnimationState::Walk => write!(f, "Walk"),
        }
    }
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Right,
    Up,
    Left,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

#[derive(Reflect)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current: usize,
    pub speed: f32,
    pub timer: Timer,
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            current: 0,
            speed: 0.1,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

impl PartialEq for Animation {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames
            && self.current == other.current
            && (self.speed - other.speed) < EPSILON
            && self.timer.duration() == other.timer.duration()
    }
}

// Perhaps this could be BillboardEntity or something?
#[derive(Component, Reflect)]
pub struct AnimatedCharacter {
    // The orientation that the character is facing
    pub heading: Vec3,
    // The direction the character is shown, from camera's perspective
    pub direction: Direction,
    // What animation the character is performing
    pub animation_state: AnimationState,
    #[reflect(ignore)]
    pub animations: HashMap<(AnimationState, Direction), Animation>,
}

impl Default for AnimatedCharacter {
    fn default() -> Self {
        Self {
            heading: Vec3::Z,
            direction: Direction::Down,
            animation_state: AnimationState::Idle,
            animations: HashMap::new(),
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct TurnTowardCamera(pub bool);
