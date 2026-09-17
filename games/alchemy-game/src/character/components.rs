use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
#[require(MoveIntent, MoveSpeed, RigidBody::Kinematic, Transform, Visibility)]
pub struct Character;

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub struct MoveIntent {
    pub(super) dir: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MoveSpeed(pub(super) f32);

impl MoveSpeed {
    pub const HUMAN: Self = Self(65.0);
}

impl Default for MoveSpeed {
    fn default() -> Self {
        MoveSpeed(100.0)
    }
}

impl MoveIntent {
    pub const STILL: Self = Self { dir: Vec2::ZERO };

    pub fn new(dir: Vec2) -> Self {
        Self {
            dir: dir.clamp_length_max(1.0),
        }
    }

    /// The normalized-ish movement direction, zero when standing still.
    pub fn dir(&self) -> Vec2 {
        self.dir
    }

    pub fn is_moving(&self) -> bool {
        self.dir != Vec2::ZERO
    }
}
