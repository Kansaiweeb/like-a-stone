use super::{Map, Position, Rect, TileType};
use bevy::prelude::*;
mod bsp_dungeon;
mod common;
use bsp_dungeon::BspDungeonBuilder;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, commands: &mut Commands);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

// pub fn build_random_map(new_depth: i32) -> (Map, Position) {
//     SimpleMapBuilder::build(new_depth)
// }

// pub fn spawn(map: &mut Map, ecs: &mut World, new_depth: i32) {
//     SimpleMapBuilder::spawn(map, ecs, new_depth);
// }

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    Box::new(BspDungeonBuilder::new(new_depth))
}
