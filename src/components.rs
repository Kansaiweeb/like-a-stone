use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Component, Clone)]
pub struct ViewShed {
    pub visible_tiles: Vec<Entity>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct TileCollider;
