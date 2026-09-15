use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::character::components::MoveIntent;
use crate::player::components::Move;
use crate::player::components::PlayerMarker;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerMarker,
        actions!(
            PlayerMarker[(
                Action::<Move>::new(),
                DeadZone::default(),
                Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick()))
            )]
        ),
    ));
}

pub fn on_move(event: On<Fire<Move>>, mut intent: Query<&mut MoveIntent>) {
    if let Ok(mut intent) = intent.get_mut(event.context) {
        *intent = MoveIntent::new(event.value);
    }
}

pub fn on_move_stop(event: On<Complete<Move>>, mut intent: Query<&mut MoveIntent>) {
    if let Ok(mut intent) = intent.get_mut(event.context) {
        *intent = MoveIntent::STILL;
    }
}
