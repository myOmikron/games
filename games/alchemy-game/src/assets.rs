use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::assets::LdtkProject;

#[derive(AssetCollection, Resource)]
pub struct MainAssets {
    #[asset(path = "main.ldtk")]
    pub main_level: Handle<LdtkProject>,

    #[asset(texture_atlas_layout(tile_size_x = 24, tile_size_y = 24, columns = 4, rows = 4))]
    pub player_layout: Handle<TextureAtlasLayout>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 4))]
    pub player_attack_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "tilesets/player/idle.png")]
    pub player_idle: Handle<Image>,

    #[asset(path = "tilesets/player/walk.png")]
    pub player_walk: Handle<Image>,

    #[asset(path = "tilesets/player/attack.png")]
    pub player_attack: Handle<Image>,

    #[asset(path = "tilesets/player/dead.png")]
    pub player_dead: Handle<Image>,
}

/// The player's animation sheets. Each pairs an image with the layout that
/// describes it, so the two cannot be mismatched at the call site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerSheet {
    Idle,
    Walk,
    Attack,
    Dead,
}

impl PlayerSheet {
    /// Frames per animation, i.e. the number of columns in every sheet.
    pub const FRAMES: usize = 4;
}

impl MainAssets {
    /// The image and the layout describing it, which always belong together.
    pub fn player_sheet(
        &self,
        sheet: PlayerSheet,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match sheet {
            PlayerSheet::Idle => (&self.player_idle, &self.player_layout),
            PlayerSheet::Walk => (&self.player_walk, &self.player_layout),
            PlayerSheet::Dead => (&self.player_dead, &self.player_layout),
            PlayerSheet::Attack => (&self.player_attack, &self.player_attack_layout),
        }
    }

    /// Builds a sprite for `sheet`, showing `frame` of `facing`.
    ///
    /// Every sheet is a 4x4 grid: one column per animation frame, one row per
    /// facing direction.
    pub fn player_sprite(&self, sheet: PlayerSheet, facing: usize, frame: usize) -> Sprite {
        let (image, layout) = self.player_sheet(sheet);

        Sprite::from_atlas_image(
            image.clone(),
            TextureAtlas {
                layout: layout.clone(),
                index: facing * PlayerSheet::FRAMES + frame,
            },
        )
    }
}
