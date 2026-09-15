use avian2d::prelude::*;
use bevy::prelude::*;

use crate::character::components::MoveIntent;
use crate::character::components::MoveSpeed;

pub(in crate::character) fn apply_move_intent(
    mut query: Query<(&mut LinearVelocity, &MoveIntent, &MoveSpeed)>,
) {
    for (mut linear_vel, intent, speed) in query.iter_mut() {
        linear_vel.0 = intent.dir + speed.0;
    }
}
