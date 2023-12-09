use bevy::prelude::*;

pub const MAIN_TEXTURE_TILE_WIDTH: f32 = 32.;
pub const MAIN_TEXTURE_TILE_HEIGHT: f32 = 32.;

pub const MAIN_TEXTURE_COLUMNS: usize = 16;
pub const MAIN_TEXTURE_ROWS: usize = 16;

#[derive(Resource)]
pub struct MainTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct MainAtlas(pub Handle<TextureAtlas>);

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub viewport: Vec2,
    pub world: Vec2,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct RootNode;

#[derive(Component)]
pub struct Tooltip;

#[derive(Component)]
pub struct TooltipText;

#[derive(Component)]
pub struct PlayerHealthLabel;
