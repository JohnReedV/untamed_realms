use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use crate::resources::*;
pub use components::*;
pub use resources::*;
use systems::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Weather>()
            .init_resource::<WorldState>()
            .init_resource::<LanguageModelAPI>()
            .init_resource::<PlayerAnimationTimer>()
            .init_resource::<NPCInteractionState>()
            .add_systems(OnEnter(GameState::Menu), despawn_npc)
            .add_systems(
                Update,
                (
                    npc_interaction_system.run_if(in_state(GameState::Game)),
                    animate_npc.run_if(in_state(GameState::Game)),
                    npc_interaction_ui_system.run_if(in_state(GameState::Game)),
                ),
            );
    }
}
