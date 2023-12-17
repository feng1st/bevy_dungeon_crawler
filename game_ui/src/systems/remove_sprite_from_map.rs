use bevy::prelude::*;
use game_core::prelude::*;

pub fn remove_sprite_from_map(
    mut commands: Commands,
    mut removed_map_pos: RemovedComponents<MapPos>,
    entity_query: Query<Entity>,
) {
    for entity in removed_map_pos.read() {
        if entity_query.get(entity).is_ok() {
            commands.entity(entity).remove::<SpriteSheetBundle>();
        }
    }
}
