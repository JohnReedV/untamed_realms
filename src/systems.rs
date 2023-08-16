use bevy::prelude::*;
use crate::components::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, PlayerCamera {}));
}