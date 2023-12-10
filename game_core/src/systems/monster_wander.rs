use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn monster_wander(
    mut move_figure_event_writer: EventWriter<MoveAction>,
    query: Query<(Entity, &MapPos), (With<Monster>, With<WanderingMind>)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, map_pos) in &query {
        let delta = match rng.gen_range(0..20) {
            0 => IVec2::new(0, 1),
            1 => IVec2::new(0, -1),
            2 => IVec2::new(-1, 0),
            3 => IVec2::new(1, 0),
            _ => IVec2::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            move_figure_event_writer.send(MoveAction {
                actor: entity,
                target_pos: map_pos.0 + delta,
            });
        }
    }
}
