use super::RESOLUTION;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            top: 1.0,
            bottom: -1.0,
            left: 1.0 * RESOLUTION,
            right: -1.0 * RESOLUTION,
            ..default()
        },
        ..default()
    };

    commands.spawn_bundle(camera);
}
