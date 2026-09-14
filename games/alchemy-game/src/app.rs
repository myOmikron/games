use bevy::app::App;
use bevy::prelude::AppExtStates;
use bevy::prelude::Plugin;

use crate::camera::CameraPlugin;
use crate::states::GameState;
use crate::states::PlayMode;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin)
            .insert_state(GameState::default())
            .insert_state(PlayMode::default());
    }
}
