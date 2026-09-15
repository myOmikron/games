use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputAction;

use crate::character::components::Character;

#[derive(Component, Reflect, Default, Clone, Debug)]
#[require(Character)]
pub struct PlayerMarker;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;
