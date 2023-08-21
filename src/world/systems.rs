use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    resources::*,
    world::{components::*, resources::*},
    npc::{NpcAnimationTimer, NpcAnimationFrame},
    NPCIdentityComponent, NPCStateComponent, PickableBundle,
};

pub const CHUNK_SIZE: i32 = 10;
pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_SIZE: i32 = (CHUNK_SIZE * (TILE_SIZE as i32) + 15) / 16 * 16;

pub fn create_world(
    commands: Commands,
    mut reader: EventReader<GameStart>,
    assets: Res<AssetServer>,
    mut the_world: ResMut<TheWorld>,
) {
    if let Some(_game_start) = reader.iter().last() {
        let world = generate_world();
        the_world.new(world.clone());
        render_world(commands, world, assets);
    }
}

pub fn despawn_world(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    world_query: Query<Entity, With<Tile>>,
) {
    if let Some(_game_over) = reader.iter().last() {
        for world_entity in world_query.iter() {
            commands.entity(world_entity).despawn_recursive();
        }
    }
}

fn render_world(mut commands: Commands, world: Vec<Vec<Tile>>, assets: Res<AssetServer>) {
    let floor_handle: Handle<Image> = assets.load("sprites/grass.png").into();
    let npc_handle: Handle<Image> = assets.load("sprites/npcs/idle/npc1.png");

    let mut id = 1;
    for row in world {
        for tile in row {
            if tile.data.npc {
                id += 1;

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.2),
                        texture: npc_handle.clone(),
                        ..default()
                    },
                    NPCIdentityComponent {
                        id: id,
                        name: "Joe".to_string(),
                    },
                    NPCStateComponent {
                        health: 100.0,
                        position: Vec3::new(tile.pos.x, tile.pos.y, 0.0),
                    },
                    PickableBundle::default(),
                    NpcAnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                    NpcAnimationFrame {current_frame: 0}
                ));
            }

            commands.spawn((
                SpriteBundle {
                    texture: floor_handle.clone(),
                    transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.0),
                    ..default()
                },
                tile,
            ));
        }
    }
}

fn generate_world() -> Vec<Vec<Tile>> {
    let mut world = Vec::new();
    let mut rng = rand::thread_rng();
    const NPC_SPAWN_CHANCE: f32 = 0.0005;

    for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
        for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let mut row = Vec::new();
                for y in 0..CHUNK_SIZE {
                    row.push(Tile {
                        pos: Position {
                            x: ((chunk_x * CHUNK_SIZE + x) as f32 - WORLD_SIZE as f32 / 2.0) * TILE_SIZE,
                            y: ((chunk_y * CHUNK_SIZE + y) as f32 - WORLD_SIZE as f32 / 2.0) * TILE_SIZE,
                        },
                        data: TileData {
                            npc: rng.gen::<f32>() < NPC_SPAWN_CHANCE,
                        },
                    });
                }
                world.push(row);
            }
        }
    }

    world
}
