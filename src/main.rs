use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod menu;
mod world;
mod systems;
mod resources;
mod player;
mod components;
pub mod npc;

use menu::MainMenuPlugin;
use systems::*;
use world::*;
use resources::*;
use player::PlayerPlugin;
pub use npc::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Untamed Realms".into(),
                    resolution: (1920., 1080.).into(),
                    mode: WindowMode::Fullscreen,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            MainMenuPlugin,
            WorldPlugin,
            PlayerPlugin,
            NpcPlugin,
        ))
        .add_event::<GameStart>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .run();
}