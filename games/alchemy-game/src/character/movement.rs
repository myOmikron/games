use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub struct MoveIntent {
    dir: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MoveSpeed(f32);

impl MoveSpeed {
    pub const HUMAN: Self = Self(100.0);
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

    pub fn is_moving(&self) -> bool {
        self.dir != Vec2::ZERO
    }
}

pub(in crate::character) fn apply_move_intent(
    mut query: Query<(&mut LinearVelocity, &MoveIntent, &MoveSpeed)>,
) {
    for (mut linear_vel, intent, speed) in query.iter_mut() {
        linear_vel.0 = intent.dir + speed.0;
    }
}
