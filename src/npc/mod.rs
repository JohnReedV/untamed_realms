use bevy::prelude::*;

pub mod components;
pub mod resources;

pub use components::*;
pub use resources::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Weather>()
        .init_resource::<WorldState>()
        .init_resource::<LanguageModelAPI>();
    }
}