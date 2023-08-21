use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct NPCIdentityComponent {
    pub id: u32,
    pub name: String,
}

#[derive(Component)]
pub struct NPCMemoryComponent {
    pub events: VecDeque<MemoryEvent>,
}

#[derive(Component)]
pub struct MemoryEvent {
    pub event_type: String,
    pub details: String,
    pub timestamp: f64,
}

#[derive(Component)]
pub struct NPCLanguageModelComponent {
    pub api_key: String,
    pub context: String,
}

#[derive(Component)]
pub struct NPCStateComponent {
    pub health: f32,
    pub position: Vec3,
}

#[derive(Component)]
pub enum StatusEffect {
    Poisoned,
    Stunned,
}

#[derive(Component)]
pub struct NPCSkillsComponent {
    pub skills: Vec<Skill>,
}

#[derive(Component)]
pub enum Skill {
    Movement,
    Attack,
    Persuasion,
}

#[derive(Component)]
pub struct NpcButton {}

#[derive(Component)]
pub struct PlayerAnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct NpcAnimationTimer(pub Timer);

#[derive(Component)]
pub struct NpcAnimationFrame {
    pub current_frame: usize,
}
