use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_enhanced_input::prelude::InputAction;

use crate::character::components::Character;

#[derive(Component, Reflect, Default, Clone, Debug)]
#[require(Character)]
pub struct PlayerMarker;

#[derive(LdtkEntity, Reflect, Clone, Debug, Default, Bundle)]
pub struct Player {
    pub player: PlayerMarker,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;
