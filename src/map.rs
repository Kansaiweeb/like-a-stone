use super::{AsciiSheet, TILE_SIZE};
use crate::ascii::spawn_ascii_sprite;
use crate::components::{Tile, TileCollider};
use bevy::prelude::*;
use bevy::utils::HashSet;

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,
    pub tile_content: Vec<Vec<Entity>>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

impl Map {
    pub fn new(new_depth: i32) -> Map {
        Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            depth: new_depth,
            bloodstains: HashSet::new(),
            tile_content: vec![Vec::new(); MAPCOUNT],
        }
    }
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall
        }
    }
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

pub fn draw_map(map: ResMut<Map>, mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut y = 0;
    let mut x = 0;

    let mut tiles = Vec::new();
    for tile in map.tiles.iter() {
        // Render a tile depending upon the tile type
        let char = match tile {
            TileType::Floor => '.',
            TileType::Wall => '#',
            _ => '.',
        };
        // warn!("{}", char);
        let tile = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            char as usize,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
            Vec3::new(
                x as f32 * 32.0 * TILE_SIZE,
                -(y as f32) * TILE_SIZE * 32.0,
                32.0,
            ),
        );
        if char == '#' {
            commands.entity(tile).insert(TileCollider);
        }
        commands
            .entity(tile)
            .insert(Visibility { is_visible: false })
            .insert(Tile {});
        // warn!("{:?}", ent);
        // warn!("{:?}", id);
        tiles.push(tile);

        // Move the coordinates
        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
    commands
        .spawn()
        .insert(Name::new("Tiles"))
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .push_children(&tiles);
}
