use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tile {
    pub pos: Position,
    pub data: TileData
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct TileData {
    pub npc: bool
}