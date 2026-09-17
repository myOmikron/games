use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::app::PluginGroup;
use bevy::image::ImagePlugin;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_enoki::EnokiPlugin;
use bevy_firefly::app::FireflyPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;

use crate::app::GamePlugin;

pub mod app;
pub mod assets;
pub mod camera;
pub mod character;
pub mod player;
pub mod states;
pub mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            EnokiPlugin,
            AudioPlugin,
            FireflyPlugin,
            TweeningPlugin,
            EnhancedInputPlugin,
            YarnSpinnerPlugin::new(),
            PhysicsPlugins::default(),
        ));

    #[cfg(feature = "dev")]
    {
        use bevy_inspector_egui::bevy_egui::EguiPlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    }

    app.add_plugins(GamePlugin).run();
}
