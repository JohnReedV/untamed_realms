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
    pub position: Vec2,
    pub status_effects: Vec<StatusEffect>,
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