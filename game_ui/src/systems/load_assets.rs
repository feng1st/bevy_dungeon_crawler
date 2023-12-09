use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dungeonfont.png");
    let atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(MAIN_TEXTURE_TILE_WIDTH, MAIN_TEXTURE_TILE_HEIGHT),
        MAIN_TEXTURE_COLUMNS,
        MAIN_TEXTURE_ROWS,
        None,
        None,
    ));
    commands.insert_resource(MainTexture(texture_handle));
    commands.insert_resource(MainAtlas(atlas_handle));
}
