use crate::npc::components::*;
use crate::resources::*;
use crate::NPCInteractionState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_mod_picking::events::{Click, Pointer};

pub fn despawn_npc(
    mut reader: EventReader<GameOver>,
    npc_query: Query<Entity, With<NPCIdentityComponent>>,
    mut commands: Commands,
) {
    if let Some(_game_over) = reader.iter().last() {
        for npc_entity in npc_query.iter() {
            commands.entity(npc_entity).despawn_recursive();
        }
    }
}

pub fn npc_interaction_system(
    mut events: EventReader<Pointer<Click>>,
    npcs: Query<&NPCIdentityComponent>,
    mut interaction_state: ResMut<NPCInteractionState>,
) {
    for event in events.iter() {
        let entity = event.target;
        if let Ok(_identity) = npcs.get(entity) {
            interaction_state.active = true;
        }
    }
}

pub fn npc_interaction_ui_system(
    mut egui_ctx: Query<&mut EguiContext>,
    mut interaction_state: ResMut<NPCInteractionState>,
    windows: Query<&mut Window>,
) {
    let mut egui_ctx = egui_ctx.single_mut();
    let ctx = egui_ctx.get_mut();

    if interaction_state.active {
        let window: &Window = windows.single();
        let egui_window_size = egui::Vec2::new(200.0, 100.0);

        let pos = egui::Pos2::new(
            (window.width() - egui_window_size.x) / 2.0,
            (window.height() - egui_window_size.y) / 2.0,
        );

        egui::Window::new("Interaction").default_pos(pos).show(ctx, |ui| {
            ui.label("Enter your text:");
            ui.text_edit_singleline(&mut interaction_state.text);
            if ui.button("Submit").clicked() {
                println!("User input: {}", interaction_state.text);
                interaction_state.active = false;
            }
        });
    }
}

pub fn animate_npc(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut sprite_query: Query<(&mut Handle<Image>, &mut Sprite), With<NPCIdentityComponent>>,
    mut animation_query: Query<(&mut NpcAnimationTimer, &mut NpcAnimationFrame)>,
) {
    let texture_paths = [
        "sprites/npcs/idle/npc1.png",
        "sprites/npcs/idle/npc2.png",
        "sprites/npcs/idle/npc3.png",
    ];

    for (mut timer, mut frame) in animation_query.iter_mut() {
        for (mut sprite_material_handle, _) in sprite_query.iter_mut() {
            timer.0.tick(time.delta());

            if timer.0.finished() {
                frame.current_frame = (frame.current_frame + 1) % texture_paths.len();
                *sprite_material_handle = asset_server.load(texture_paths[frame.current_frame]);
                timer.0.reset();
            }
        }
    }
}

// pub fn npc_click_detection_system(
//     mut window_query: Query<&Window>,
//     mut camera_query: Query<&Transform, With<PlayerCamera>>,
//     cursor_query: Query<&Transform, With<GameCursor>>,
//     mut npc_query: Query<(&Transform, &NPCIdentityComponent), With<NPCIdentityComponent>>,
// ) {
//     if let Ok(game_cursor) = cursor_query.get_single() {
//         if let Ok(window) = window_query.get_single() {
//             let cursor_position = game_cursor.translation;

//             for (npc_transform, npc_identity) in npc_query.iter_mut() {
//                 let npc_position = npc_transform.translation;

//                 let camera_transform = camera_query.single_mut();

//                 let camera_position = camera_transform.translation;
//                 let difference = camera_position - cursor_position;
//                 let final_cursor_position = camera_position + difference;

//                 println!(
//                     "Camera Positon {}, Difference: {}, Cursor Position: {} Distance: {}",
//                     camera_position / 32.0,
//                     difference / 32.0,
//                     final_cursor_position / 32.0,
//                     final_cursor_position.distance(npc_position) /32.0
//                 );

//                 if final_cursor_position.distance(npc_position) < 32.0 {
//                     println!("Clicked on or hovered over NPC: {}", npc_identity.id);
//                 }
//             }
//         }
//     }
// }
