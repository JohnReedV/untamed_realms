use bevy::prelude::*;

mod components;
mod systems;

use crate::resources::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), create_world)
            .add_systems(OnEnter(GameState::Menu), despawn_world);
    }
}
