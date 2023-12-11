use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala_sprite(
    mut commands: Commands,
    amulet_query: Query<Entity, With<AmuletOfYala>>,
    main_atlas: Res<MainAtlas>,
) {
    if let Ok(entity) = amulet_query.get_single() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture_atlas: main_atlas.0.clone(),
            sprite: TextureAtlasSprite::new('|' as usize),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }),
            ..default()
        });
    }
}
