use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub pos: Position,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}
