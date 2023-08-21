use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod components;
mod menu;
pub mod npc;
mod player;
mod resources;
mod systems;
mod world;

pub use bevy_mod_picking::{
    prelude::{DebugPickingPlugin, PickableBundle, RaycastPickCamera, RaycastPickTarget},
    DefaultPickingPlugins,
};
use menu::MainMenuPlugin;
pub use npc::*;
use player::PlayerPlugin;
use resources::*;
use systems::*;
use world::*;

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
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
        ))
        .add_event::<GameStart>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .run();
}
