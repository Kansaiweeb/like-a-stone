use crate::{
    ascii::spawn_ascii_sprite,
    components::{Position, TileCollider, ViewShed},
    AsciiSheet, TILE_SIZE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player.label("spawn_player"))
            .add_system(player_movement.label("player_movement"))
            .add_system(camera_follow.after("player_movement"));
    }
}

fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut transform = player_query.single_mut();
    let mut y_delta = 0.0;
    let mut x_delta = 0.0;
    if keyboard.just_pressed(KeyCode::W)
        || keyboard.just_pressed(KeyCode::Up)
        || keyboard.just_pressed(KeyCode::Numpad8)
    {
        y_delta += TILE_SIZE * 32.0;
    }

    if keyboard.just_pressed(KeyCode::S)
        || keyboard.just_pressed(KeyCode::Down)
        || keyboard.just_pressed(KeyCode::Numpad2)
    {
        y_delta -= TILE_SIZE * 32.0;
    }
    if keyboard.just_pressed(KeyCode::A)
        || keyboard.just_pressed(KeyCode::Left)
        || keyboard.just_pressed(KeyCode::Numpad4)
    {
        x_delta -= TILE_SIZE * 32.0;
    }
    if keyboard.just_pressed(KeyCode::D)
        || keyboard.just_pressed(KeyCode::Right)
        || keyboard.just_pressed(KeyCode::Numpad6)
    {
        x_delta += TILE_SIZE * 32.0;
    }

    if (keyboard.just_pressed(KeyCode::W) && keyboard.just_pressed(KeyCode::D))
        || keyboard.just_pressed(KeyCode::Numpad9)
        || (keyboard.just_pressed(KeyCode::Up) && keyboard.just_pressed(KeyCode::Right))
    {
        x_delta += TILE_SIZE * 32.0;
        y_delta += TILE_SIZE * 32.0;
    }
    if (keyboard.just_pressed(KeyCode::W) && keyboard.just_pressed(KeyCode::A))
        || keyboard.just_pressed(KeyCode::Numpad7)
        || (keyboard.just_pressed(KeyCode::Up) && keyboard.just_pressed(KeyCode::Left))
    {
        x_delta -= TILE_SIZE * 32.0;
        y_delta += TILE_SIZE * 32.0;
    }
    if (keyboard.just_pressed(KeyCode::S) && keyboard.just_pressed(KeyCode::D))
        || keyboard.just_pressed(KeyCode::Numpad3)
        || (keyboard.just_pressed(KeyCode::Down) && keyboard.just_pressed(KeyCode::Right))
    {
        x_delta += TILE_SIZE * 32.0;
        y_delta -= TILE_SIZE * 32.0;
    }
    if (keyboard.just_pressed(KeyCode::S) && keyboard.just_pressed(KeyCode::A))
        || keyboard.just_pressed(KeyCode::Numpad1)
        || (keyboard.just_pressed(KeyCode::Down) && keyboard.just_pressed(KeyCode::Left))
    {
        x_delta -= TILE_SIZE * 32.0;
        y_delta -= TILE_SIZE * 32.0;
    }
    let target = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE),
            wall_transform.translation,
            Vec2::splat(1.0),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>, position: Res<Position>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        2,
        Color::rgb(0.9, 0.9, 0.9),
        Vec3::new(position.x as f32 * 32.0, -position.y as f32 * 32.0, 900.0),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 30.0 })
        .insert(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
        });
}

fn try_move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    let mut transform = player_query.single_mut();
}
