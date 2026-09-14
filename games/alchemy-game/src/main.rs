use bevy::DefaultPlugins;
use bevy::app::App;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    {
        use bevy_inspector_egui::WorldInspectorPlugin;
        use bevy_inspector_egui::bevy_egui::EguiPlugin;

        app.add_plugins((WorldInspectorPlugin::new(), EguiPlugin::default()));
    }

    app.run();
}
