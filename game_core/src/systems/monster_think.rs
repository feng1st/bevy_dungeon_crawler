use bevy::prelude::*;

use crate::prelude::*;

const WANDERING_DISTANCE_SQUARED: i32 = 30 * 30;

#[allow(clippy::type_complexity)]
pub fn monster_think(
    mut commands: Commands,
    monster_query: Query<
        (
            Entity,
            &MapPos,
            &FieldOfView,
            Option<&WanderingMind>,
            Option<&ChasingMind>,
        ),
        (With<Monster>, With<SimpleAI>),
    >,
    player_query: Query<&MapPos, (With<Player>, Without<Monster>)>,
) {
    let player_pos = player_query.get_single().map(|x| x.0);

    for (entity, pos, field_of_view, wandering_mind, chasing_mind) in &monster_query {
        let mut entity_commands = commands.entity(entity);

        // init wandering
        if wandering_mind.is_none() && chasing_mind.is_none() {
            entity_commands.insert(WanderingMind);
            continue;
        }

        if let Ok(player_pos) = player_pos {
            // player is in sight
            if field_of_view.visible_tiles.contains(&player_pos) {
                // switch to chasing
                if wandering_mind.is_some() {
                    entity_commands.remove::<WanderingMind>();
                }
                if chasing_mind.is_none() {
                    entity_commands.insert(ChasingMind);
                }
            }
            // player is far away
            else if pos.0.distance_squared(player_pos) > WANDERING_DISTANCE_SQUARED {
                // switch to wandering
                if chasing_mind.is_some() {
                    entity_commands.remove::<ChasingMind>();
                }
                if wandering_mind.is_none() {
                    entity_commands.insert(WanderingMind);
                }
            }
        }
        // player is not present yet
        else {
            // switch to wandering
            if chasing_mind.is_some() {
                entity_commands.remove::<ChasingMind>();
            }
            if wandering_mind.is_none() {
                entity_commands.insert(WanderingMind);
            }
        }
    }
}
