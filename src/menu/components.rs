use bevy::prelude::*;


#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Component)]
pub struct OptionsButton {}

#[derive(Component)]
pub struct GameCursor {
    pub despawned: bool
}

#[derive(Component)]
pub struct Cords {}

#[derive(Component)]
pub struct FPS {}