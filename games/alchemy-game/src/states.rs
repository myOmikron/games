use bevy::prelude::States;
use bevy::prelude::SubStates;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Playing)]
pub enum PlayMode {
    #[default]
    Explore,
    Dialogue,
    Menu,
}
