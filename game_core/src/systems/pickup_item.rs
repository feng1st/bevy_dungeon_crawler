use bevy::prelude::*;

use crate::prelude::*;

pub fn pickup_item(
    mut commands: Commands,
    entity_query: Query<Entity>,
    item_query: Query<(Entity, &MapPos), With<Item>>,
    mut pickup_event_reader: EventReader<PickupAction>,
) {
    for action in pickup_event_reader.read() {
        // actor is gone
        if entity_query.get(action.actor).is_err() {
            continue;
        }

        for (item_entity, item_pos) in &item_query {
            if item_pos.0 == action.target_pos {
                commands
                    .entity(item_entity)
                    .remove::<MapPos>()
                    .insert(Carried(action.actor));
                break;
            }
        }
    }
}
