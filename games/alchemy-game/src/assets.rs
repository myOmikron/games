use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::assets::LdtkProject;

#[derive(AssetCollection, Resource)]
pub struct MainAssets {
    #[asset(path = "main.ldtk")]
    pub main_level: Handle<LdtkProject>,
}
