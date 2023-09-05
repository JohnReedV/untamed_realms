use bevy::prelude::*;
use crate::components::*;
use crate::RaycastPickCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, PlayerCamera {}, RaycastPickCamera::default()));
    dotenv::dotenv().ok();
}