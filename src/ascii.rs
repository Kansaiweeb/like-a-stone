use super::TILE_SIZE;
use bevy::prelude::*;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics);
    }
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image: Handle<Image> = assets.load("Phoebus_32x32_Next.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.0), 16, 16);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                scale: Vec3::splat(TILE_SIZE),
                ..default()
            },
            ..default()
        })
        .id()
}
