use bevy::app::App;
use bevy::prelude::AppExtStates;
use bevy::prelude::Plugin;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::prelude::ConfigureLoadingState;
use bevy_asset_loader::prelude::LoadingState;

use crate::assets::MainAssets;
use crate::camera::CameraPlugin;
use crate::character::CharacterPlugin;
use crate::player::PlayerPlugin;
use crate::states::GameState;
use crate::states::PlayMode;
use crate::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, CharacterPlugin, PlayerPlugin, WorldPlugin))
            .insert_state(GameState::default())
            .insert_state(PlayMode::default())
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Playing)
                    .load_collection::<MainAssets>(),
            );
    }
}
