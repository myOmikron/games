use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputContextAppExt;

use crate::states::GameState;

pub mod animation;
pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<components::PlayerMarker>()
            .add_observer(systems::bind_player_input)
            .add_observer(systems::on_move)
            .add_observer(systems::on_move_stop)
            .add_systems(
                Update,
                animation::animate_player.run_if(in_state(GameState::Playing)),
            );
    }
}
