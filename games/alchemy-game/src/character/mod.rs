use bevy::app::App;
use bevy::app::FixedUpdate;
use bevy::app::Plugin;

pub mod components;
pub mod movement;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, movement::apply_move_intent);
    }
}
