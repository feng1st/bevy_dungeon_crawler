use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

pub fn spawn_monster_sprites(
    mut commands: Commands,
    monster_query: Query<(Entity, &Monster)>,
    main_atlas: Res<MainAtlas>,
) {
    for (entity, monster_id) in &monster_query {
        let (_, sprite_index, _) = MONSTER_TABLE[monster_id.0];
        commands.entity(entity).insert(SpriteSheetBundle {
            texture_atlas: main_atlas.0.clone(),
            sprite: TextureAtlasSprite::new(sprite_index),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 2.,
            }),
            visibility: Visibility::Hidden,
            ..default()
        });
    }
}
