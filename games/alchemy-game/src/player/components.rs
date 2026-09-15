use avian2d::prelude::LinearVelocity;
use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputAction;

use crate::character::movement::MoveIntent;
use crate::character::movement::MoveSpeed;

#[derive(Component, Default, Reflect)]
pub struct PlayerMarker;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;

#[derive(Bundle)]
pub struct Player {
    pub player: PlayerMarker,
    pub move_intent: MoveIntent,
    pub move_speed: MoveSpeed,
    pub linear_velocity: LinearVelocity,
    pub rigidbody: RigidBody,
    pub transform: Transform,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            player: PlayerMarker,
            move_intent: MoveIntent::STILL,
            move_speed: MoveSpeed::HUMAN,
            linear_velocity: LinearVelocity::ZERO,
            rigidbody: RigidBody::Kinematic,
            transform: Transform::default(),
        }
    }
}
