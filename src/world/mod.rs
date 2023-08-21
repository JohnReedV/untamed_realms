use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;

use crate::resources::*;
use crate::world::resources::TheWorld;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TheWorld>()
        .add_systems(OnEnter(GameState::Game), create_world)
            .add_systems(OnEnter(GameState::Menu), despawn_world);
    }
}
