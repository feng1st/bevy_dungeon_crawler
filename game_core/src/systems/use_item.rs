use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn use_item(
    mut commands: Commands,
    entity_query: Query<Entity>,
    mut health_query: Query<&mut Health>,
    mut map_query: Query<&mut MapVisibilityGrid>,
    mut tile_visility_event_writer: EventWriter<TileVisibilityEvent>,
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
            if let Ok((healing, dungeon_map)) = effect_query.get(item_entity) {
                if let Some(healing) = healing {
                    if let Ok(mut health) = health_query.get_mut(action.actor) {
                        health.current = i32::min(health.current + healing.amount, health.max);
                    }
                }

                if dungeon_map.is_some() {
                    if let Ok(mut map_visibility_grid) = map_query.get_single_mut() {
                        for x in 0..map_visibility_grid.bound.x {
                            for y in 0..map_visibility_grid.bound.y {
                                let pos = IVec2::new(x, y);
                                if map_visibility_grid.get_at(pos) == Some(TileVisibility::Hidden) {
                                    map_visibility_grid.set_at(pos, TileVisibility::Revealed);
                                    tile_visility_event_writer.send(TileVisibilityEvent {
                                        pos,
                                        visibility: TileVisibility::Revealed,
                                    });
                                }
                            }
                        }
                    }
                }

                commands.entity(item_entity).despawn();
            }
        }
    }
}
