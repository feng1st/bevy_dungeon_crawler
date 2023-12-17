use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

pub fn spawn_item_sprites(
    mut commands: Commands,
    item_query: Query<(Entity, &Item)>,
    main_atlas: Res<MainAtlas>,
) {
    for (entity, item_id) in &item_query {
        let (_, sprite_index, _) = ITEM_TABLE[item_id.0];
        commands.entity(entity).insert(SpriteSheetBundle {
            texture_atlas: main_atlas.0.clone(),
            sprite: TextureAtlasSprite::new(sprite_index),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }),
            visibility: Visibility::Hidden,
            ..default()
        });
    }
}
