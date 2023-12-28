use bevy::prelude::*;

use crate::prelude::*;

pub fn take_damage(
    mut victim_query: Query<(&mut Health, Option<&mut Rest>, &MapPos, Option<&Player>)>,
    mut map_query: Query<&mut MapFigureGrid>,
    mut commands: Commands,
    mut apply_damage_event_reader: EventReader<AttackAction>,
) {
    for action in apply_damage_event_reader.read() {
        let Ok((mut victim_health, victim_rest, victim_pos, victim_is_player)) =
            victim_query.get_mut(action.target)
        else {
            continue;
        };

        victim_health.current -= action.damage;
        // interrupted
        if let Some(mut victim_rest) = victim_rest {
            victim_rest.current = 0;
        }
        if victim_health.current <= 0 {
            // kill monster
            if victim_is_player.is_none() {
                if let Ok(mut map_figure_grid) = map_query.get_single_mut() {
                    map_figure_grid.reset(victim_pos.0);
                }
                commands.entity(action.target).despawn();
            }
        }
    }
}
