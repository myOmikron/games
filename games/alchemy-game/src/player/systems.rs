use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::character::components::MoveIntent;
use crate::player::components::Move;
use crate::player::components::PlayerMarker;

/// Attaches the input bindings to whichever entity gains [`PlayerMarker`].
///
/// The player itself is placed in LDtk, so it appears only once the level has
/// been processed -- an observer catches it whenever that happens.
pub fn bind_player_input(add: On<Add, PlayerMarker>, mut commands: Commands) {
    commands.entity(add.entity).insert(actions!(
        PlayerMarker[(
            Action::<Move>::new(),
            DeadZone::default(),
            Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick()))
        )]
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
