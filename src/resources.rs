use bevy::prelude::*;

#[derive(Event)]
pub struct GameStart {}

#[derive(Event)]
pub struct GameOver {}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Paused,
    Game,
}
