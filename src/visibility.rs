use bevy::prelude::*;
use bevy::render::view::VisibleEntities;

use crate::components::{Tile, ViewShed};
use crate::map::Map;

use bevy::prelude::*;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(visibility_system);
    }
}

fn visibility_system(
    mut commands: Commands,
    mut viewsheds_positions: Query<(&mut ViewShed, &Transform)>,
    map: ResMut<Map>,
    map_tiles: Query<(Entity), With<Tile>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut viewshed, transform) in viewsheds_positions.iter_mut() {
        viewshed.visible_tiles.clear();
        let x = (transform.translation.x / 32.0) as i32;
        let y = (transform.translation.y / -32.0) as i32;
        let idx = map.xy_idx(x, y) + 2;
        let mut visible_tiles = Vec::new();
        for i in -2..3 {
            for j in -2..3 as i32 {
                visible_tiles.push(idx + (j + map.width * i) as usize)
            }
        }
        // warn!("{:?}", visible_tiles);
        // let mut iter = map_tiles.iter_combinations();
        // while let Some([(tile, handle)]) = iter.fetch_next() {
        for tile in map_tiles.iter() {
            // let color = &mut materials.get_mut(handle).unwrap().color;
            // warn!("{:?}", tile.id());
            if visible_tiles.contains(&(tile.id() as usize)) {
                commands
                    .entity(tile)
                    .insert(Visibility { is_visible: true });
                // warn!("changed {}", tile.id());
            }
        }
    }
}
