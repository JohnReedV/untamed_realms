use bevy::prelude::*;
use crate::world::components::*;

#[derive(Resource, Clone)]
pub struct TheWorld {
    pub world: Vec<Vec<Tile>>,
}

impl Default for TheWorld {
    fn default() -> Self {
        TheWorld {
            world: Vec::new(),
        }
    }
}

impl TheWorld {
    pub fn new(&mut self, world: Vec<Vec<Tile>>) {
        self.world = world;
    }
}