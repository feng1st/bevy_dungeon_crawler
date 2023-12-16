use bevy::prelude::*;

use crate::prelude::*;

pub fn use_item(
    mut commands: Commands,
    entity_query: Query<Entity>,
    mut health_query: Query<&mut Health>,
    item_query: Query<(Entity, &Carried), With<Item>>,
    effect_query: Query<(Option<&ProvidesHealing>, Option<&ProvidesDungeonMap>)>,
    mut use_item_event_reader: EventReader<UseItemAction>,
) {
    for action in use_item_event_reader.read() {
        // actor is gone
        if entity_query.get(action.actor).is_err() {
            continue;
        }

        if let Some(item_entity) = item_query
            .iter()
            .filter(|(_, carried)| carried.0 == action.actor)
            .enumerate()
            .filter(|(no, _)| *no == action.target_index)
            .map(|(_, (item_entity, _))| item_entity)
            .next()
        {
            if let Ok((healing, _dungeon_map)) = effect_query.get(item_entity) {
                if let Some(healing) = healing {
                    if let Ok(mut health) = health_query.get_mut(action.actor) {
                        health.current = i32::min(health.current + healing.amount, health.max);
                    }
                }

                // if let Some(dungeon_map) = dungeon_map {
                //     // commands.entity(action.actor).insert(dungeon_map.0);
                // }

                commands.entity(item_entity).despawn();
            }
        }
    }
}
