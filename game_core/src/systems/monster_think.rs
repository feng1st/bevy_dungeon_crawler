use bevy::prelude::*;

use crate::prelude::*;

const CHASING_DISTANCE_SQUARED: i32 = 20 * 20;
const WANDERING_DISTANCE_SQUARED: i32 = 40 * 40;

#[allow(clippy::needless_pass_by_value)]
pub fn monster_think(
    mut commands: Commands,
    monster_query: Query<
        (
            Entity,
            &MapPos,
            Option<&WanderingMind>,
            Option<&ChasingMind>,
        ),
        (With<Monster>, With<SimpleAI>),
    >,
    player_query: Query<&MapPos, (With<Player>, Without<Monster>)>,
) {
    let mut target: Option<IVec2> = None;
    if let Ok(pos) = player_query.get_single() {
        target = Some(pos.0);
    }

    for (entity, pos, wandering_mind, chasing_mind) in &monster_query {
        let mut entity_commands = commands.entity(entity);
        if let Some(player_pos) = target {
            let distance_squared = pos.0.distance_squared(player_pos);
            if distance_squared < CHASING_DISTANCE_SQUARED {
                // switch to chasing
                if wandering_mind.is_some() {
                    entity_commands.remove::<WanderingMind>();
                }
                if chasing_mind.is_none() {
                    entity_commands.insert(ChasingMind);
                }
            } else if distance_squared > WANDERING_DISTANCE_SQUARED {
                // switch to wandering
                if chasing_mind.is_some() {
                    entity_commands.remove::<ChasingMind>();
                }
                if wandering_mind.is_none() {
                    entity_commands.insert(WanderingMind);
                }
            } else {
                // init wandering
                if wandering_mind.is_none() && chasing_mind.is_none() {
                    entity_commands.insert(WanderingMind);
                }
            }
        } else {
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
