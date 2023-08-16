use bevy::prelude::*;

mod systems;

use crate::resources::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(OnEnter(GameState::Menu), despawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Game)));
    }
}