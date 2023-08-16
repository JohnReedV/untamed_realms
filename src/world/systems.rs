use bevy::prelude::*;

use crate::resources::*;
use crate::world::components::*;

pub const CHUNK_SIZE: i32 = 20;
pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_SIZE: i32 = (CHUNK_SIZE * (TILE_SIZE as i32) + 15) / 16 * 16;

pub fn create_world(commands: Commands, mut reader: EventReader<GameStart>, assets: Res<AssetServer>) {
    if let Some(_game_start) = reader.iter().last() {
        render_world(commands, generate_world(), assets);
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
    let handle: Handle<Image> = assets.load("sprites/floor1.png").into();

    for row in world {
        for tile in row {
            commands.spawn((
                SpriteBundle {
                    texture: handle.clone(),
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
                    });
                }
                world.push(row);
            }
        }
    }

    world
}
