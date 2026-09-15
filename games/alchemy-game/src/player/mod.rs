use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputContextAppExt;

use crate::states::GameState;

pub mod components;
pub mod systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<components::PlayerMarker>()
            .add_systems(OnEnter(GameState::Playing), systems::spawn_player)
            .add_observer(systems::on_move)
            .add_observer(systems::on_move_stop);
    }
}
