use std::time::Duration;

use bevy::prelude::*;

use crate::assets::MainAssets;
use crate::assets::PlayerSheet;
use crate::character::components::MoveIntent;
use crate::player::components::PlayerMarker;

/// The direction the player is drawn facing.
///
/// The discriminants are the row indices of the player sheets, which are
/// ordered down, left, right, up from top to bottom.
#[derive(Component, Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Facing {
    #[default]
    Down,
    Left,
    Right,
    Up,
}

impl Facing {
    /// Row index of this direction in every player sheet.
    pub const fn row(self) -> usize {
        self as usize
    }

    /// The facing for a movement direction, or `None` when standing still.
    ///
    /// Diagonals resolve to the dominant axis, so the sprite keeps the
    /// direction the player is mostly heading instead of flickering.
    pub fn from_dir(dir: Vec2) -> Option<Self> {
        if dir == Vec2::ZERO {
            return None;
        }

        Some(if dir.x.abs() > dir.y.abs() {
            if dir.x > 0.0 { Self::Right } else { Self::Left }
        } else if dir.y > 0.0 {
            Self::Up
        } else {
            Self::Down
        })
    }
}

/// Cycles the frames of the player's current sheet.
#[derive(Component, Debug)]
pub struct PlayerAnimation {
    timer: Timer,
    sheet: PlayerSheet,
    frame: usize,
}

const IDLE_FRAME_TIME: f32 = 0.35;
const WALK_FRAME_TIME: f32 = 0.2;

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(IDLE_FRAME_TIME, TimerMode::Repeating),
            sheet: PlayerSheet::Idle,
            frame: 0,
        }
    }
}

impl PlayerAnimation {
    /// Switches to `sheet`, restarting the cycle at its first frame.
    fn play(&mut self, sheet: PlayerSheet) {
        if self.sheet == sheet {
            return;
        }

        let frame_time = match sheet {
            PlayerSheet::Walk => WALK_FRAME_TIME,
            _ => IDLE_FRAME_TIME,
        };

        self.sheet = sheet;
        self.frame = 0;
        self.timer.set_duration(Duration::from_secs_f32(frame_time));
        self.timer.reset();
    }
}

/// Turns the player towards its movement direction and advances the sprite.
pub fn animate_player(
    time: Res<Time>,
    assets: Res<MainAssets>,
    mut players: Query<
        (&MoveIntent, &mut Facing, &mut PlayerAnimation, &mut Sprite),
        With<PlayerMarker>,
    >,
) {
    for (intent, mut facing, mut animation, mut sprite) in &mut players {
        if let Some(new_facing) = Facing::from_dir(intent.dir()) {
            facing.set_if_neq(new_facing);
        }

        animation.play(if intent.is_moving() {
            PlayerSheet::Walk
        } else {
            PlayerSheet::Idle
        });

        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.frame = (animation.frame + 1) % PlayerSheet::FRAMES;
        }

        let (image, layout) = assets.player_sheet(animation.sheet);
        let index = facing.row() * PlayerSheet::FRAMES + animation.frame;

        // Mutate in place rather than replacing the sprite, so everything LDtk
        // set up on it survives.
        if sprite.image != *image {
            sprite.image = image.clone();
        }

        match sprite.texture_atlas.as_mut() {
            Some(atlas) => {
                if atlas.layout != *layout {
                    atlas.layout = layout.clone();
                }
                if atlas.index != index {
                    atlas.index = index;
                }
            }
            None => {
                sprite.texture_atlas = Some(TextureAtlas {
                    layout: layout.clone(),
                    index,
                });
            }
        }
    }
}
