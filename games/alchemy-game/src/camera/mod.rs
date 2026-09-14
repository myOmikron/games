use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;
use bevy::camera::Camera2d;
use bevy::color::Color;
use bevy::prelude::Commands;
use bevy::prelude::default;
use bevy_firefly::data::FireflyConfig;
use bevy_firefly::lights::PointLight2d;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((PointLight2d {
                color: Color::WHITE,
                radius: 10_000.0,
                ..default()
            },));
            commands.spawn((Camera2d, FireflyConfig::default()));
        });
    }
}
