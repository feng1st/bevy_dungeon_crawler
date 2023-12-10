use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_player_sprite(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    main_atlas: Res<MainAtlas>,
) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture_atlas: main_atlas.0.clone(),
            sprite: TextureAtlasSprite::new('@' as usize),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 2.,
            }),
            ..default()
        });
    }
}
