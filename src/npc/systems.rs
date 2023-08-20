use bevy::prelude::*;
use crate::npc::components::*;
use crate::resources::*;
use bevy_mod_picking::events::{Pointer, Click};


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
    npcs: Query<&NPCIdentityComponent>
) {
    for event in events.iter() {
        let entity = event.target;
        if let Ok(identity) = npcs.get(entity) {
            println!("Clicked on NPC: {}", identity.id);
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