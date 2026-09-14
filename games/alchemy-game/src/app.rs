use bevy::app::App;
use bevy::prelude::Plugin;

use crate::camera::CameraPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
    }
}
