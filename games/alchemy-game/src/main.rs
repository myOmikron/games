use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::app::App;
use bevy_console::ConsolePlugin;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_enoki::EnokiPlugin;
use bevy_firefly::app::FireflyPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins((
        LdtkPlugin,
        EnokiPlugin,
        AudioPlugin,
        FireflyPlugin,
        ConsolePlugin,
        TweeningPlugin,
        YarnSpinnerPlugin::new(),
        PhysicsPlugins::default(),
    ));

    #[cfg(feature = "dev")]
    {
        use bevy_inspector_egui::WorldInspectorPlugin;
        use bevy_inspector_egui::bevy_egui::EguiPlugin;

        app.add_plugins((WorldInspectorPlugin::new(), EguiPlugin::default()));
    }

    app.run();
}
