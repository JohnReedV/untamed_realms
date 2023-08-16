use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub const PLAYER_SPEED: i32 = 1000;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<GameStart>,
) {
    if let Some(_game_start) = reader.iter().last() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                texture: asset_server.load("sprites/player.png"),
                ..default()
            },
            Player {},
        ));
    }
}

pub fn despawn_player(
    mut reader: EventReader<GameOver>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if let Some(_game_over) = reader.iter().last() {
        for player_entity in player_query.iter() {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        let move_distance = direction * PLAYER_SPEED as f32 * time.delta_seconds();
        transform.translation += move_distance;

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}
