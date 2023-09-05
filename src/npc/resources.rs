use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Weather {
    #[default]
    Sunny,
    Rainy,
    Stormy,
}

#[derive(Resource, Default)]
pub struct WorldState {
    pub time: f32,
    pub weather: Weather,
    pub entities: Vec<Entity>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerAnimationTimer(Timer);

#[derive(Resource)]
pub struct NPCInteractionState {
    pub active: bool,
    pub text: String,
}

impl Default for NPCInteractionState {
    fn default() -> Self {
        Self {
            active: false,
            text: String::new(),
        }
    }
}