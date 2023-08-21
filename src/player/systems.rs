use crate::components::*;
use crate::resources::*;
use crate::PlayerAnimationIndices;
use bevy::prelude::*;

pub const PLAYER_SPEED: i32 = 1000;

pub fn spawn_player(
    asset_server: Res<AssetServer>,
    mut reader: EventReader<GameStart>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    if let Some(_game_start) = reader.iter().last() {
        let player_handle: Handle<Image> = asset_server.load("sprites/player.png");

        let sprite_size = Vec2::new(48.0, 48.0);
        let columns = 3;
        let rows = 4;
        let texture_atlas =
            TextureAtlas::from_grid(player_handle, sprite_size, columns, rows, None, None);
        let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);
        let animation_indices: PlayerAnimationIndices = PlayerAnimationIndices { first: 1, last: 2 };

        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                texture_atlas: texture_atlas_handle,
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
    mut player_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut sprite)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            sprite.index = 3;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            sprite.index = 8;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            sprite.index = 11;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            sprite.index = 2;
        }

        if keyboard_input.get_pressed().next().is_none() {
            match sprite.index {
                3 => sprite.index = 4,
                8 => sprite.index = 7,
                11 => sprite.index = 10,
                2 => sprite.index = 1,
                _ => {}
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let move_distance = direction * PLAYER_SPEED as f32 * time.delta_seconds();
        transform.translation += move_distance;

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}
