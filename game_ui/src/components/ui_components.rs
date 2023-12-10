use bevy::prelude::*;

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
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerHealthLabel;
