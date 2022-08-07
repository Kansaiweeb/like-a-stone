use bevy::{prelude::*, winit::WinitSettings};

mod ascii;
mod camera;
mod components;
mod debug;
mod map;
mod map_builders;
mod player;
mod rect;
mod visibility;

use ascii::{AsciiPlugin, AsciiSheet};
use camera::CameraPlugin;
use components::*;
use debug::DebugPlugin;
use map::{draw_map, Map, TileType};
use player::PlayerPlugin;
use rect::Rect;
use visibility::VisibilityPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 1.;
fn main() {
    let height = 720.0;
    let mut map_builder = map_builders::random_builder(1);
    map_builder.build_map();
    let map = map_builder.get_map();
    let starting_position = map_builder.get_starting_position();

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Sandbox".to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(WinitSettings::game())
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(map)
        .insert_resource(starting_position)
        .add_startup_system(draw_map)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        // .add_plugin(TileMapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(VisibilityPlugin)
        .run();
}
