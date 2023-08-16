mod components;
mod resources;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::*;
use crate::resources::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .init_resource::<FixMenuTimer>()
        .init_resource::<DrawCordsTracker>()
        .init_resource::<FpsTracker>()
        .add_systems(Startup, setup_cursor)
        .add_systems(OnEnter(GameState::Menu), spawn_main_menu)
        .add_systems(OnEnter(GameState::Paused), spawn_main_menu) 
            .add_systems(
                Update,
                (
                    fix_menu_first_game.run_if(in_state(GameState::Menu)),
                    interact_play_button,
                    interact_quit_button,
                    interact_options_button,
                    despawn_main_menu.run_if(in_state(GameState::Game)),
                    pause_game.run_if(not(in_state(GameState::Menu))),
                    move_cursor,
                    draw_cords.run_if(in_state(GameState::Game)),
                    fps_system,
                ),
            );
    }
}
