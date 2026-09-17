use bevy::prelude::*;
use bevy_firefly::data::FireflyConfig;
use bevy_firefly::lights::PointLight2d;
use tracing::info;

use crate::player::components::PlayerMarker;
use crate::states::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((PointLight2d {
                color: Color::WHITE,
                radius: 10_000.0,
                ..default()
            },));
            commands.spawn((
                Camera2d,
                Projection::Orthographic(OrthographicProjection {
                    scale: 0.25,
                    ..OrthographicProjection::default_2d()
                }),
                FireflyConfig::default(),
            ));
        })
        .add_systems(
            PostUpdate,
            follow_player
                .run_if(in_state(GameState::Playing))
                .before(TransformSystems::Propagate),
        );
    }
}

pub fn follow_player(
    player_query: Query<&GlobalTransform, With<PlayerMarker>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerMarker>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        info!("No camera was found");
        return;
    };

    let Ok(player) = player_query.single() else {
        info!("No player was found");
        return;
    };

    camera_transform.translation.x = player.translation().x;
    camera_transform.translation.y = player.translation().y;
}
