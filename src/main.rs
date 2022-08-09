use bevy::{prelude::*, winit::WinitSettings};
mod debug;

use debug::DebugPlugin;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Bevy Roguelike".to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(WinitSettings::game())
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        // .add_plugin(VisibilityPlugin)
        .run();
}
