use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_ecs_ldtk::prelude::*;

use crate::assets::MainAssets;
use crate::player::components::Player;
use crate::states::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .register_ldtk_entity_for_layer::<Player>("Entities", "Player")
            .add_systems(OnEnter(GameState::Playing), spawn_level);
    }
}

fn spawn_level(mut commands: Commands, assets: Res<MainAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: assets.main_level.clone(),
        },
        ..Default::default()
    });
}
