use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn move_figure(
    mut pos_query: Query<&mut MapPos>,
    mut rest_query: Query<&mut Rest>,
    entity_query: Query<Entity>,
    player_query: Query<&Player>,
    mut apply_damage_event_writer: EventWriter<AttackAction>,
    mut map_query: Query<(&MapTileGrid, &mut MapFigureGrid)>,
    mut move_figure_event_reader: EventReader<MoveAction>,
    amulet_res: Res<AmuletOfYalaPos>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for action in move_figure_event_reader.read() {
        // actor is gone
        if entity_query.get(action.actor).is_err() {
            continue;
        }

        let actor_is_player = player_query.get(action.actor).is_ok();

        // cancel rest
        if let Ok(mut rest) = rest_query.get_mut(action.actor) {
            rest.current = 0;
        }

        // on map
        let Ok((map_tile_grid, mut map_figure_grid)) = map_query.get_single_mut() else {
            continue;
        };

        // can enter
        if !map_tile_grid.can_enter(action.target_pos) {
            continue;
        }

        let target_entity = map_figure_grid.get(action.target_pos);
        // occupied
        if entity_query.get(target_entity).is_ok() {
            let target_is_player = player_query.get(target_entity).is_ok();
            // swap
            if actor_is_player == target_is_player {
                let mut tmp_pos: Option<IVec2> = None;
                if let Ok(mut pos) = pos_query.get_mut(action.actor) {
                    tmp_pos = Some(pos.0);
                    map_figure_grid.set(action.target_pos, action.actor);
                    pos.0 = action.target_pos;
                }
                if let (Some(actor_pos), Ok(mut pos)) = (tmp_pos, pos_query.get_mut(target_entity))
                {
                    map_figure_grid.set(actor_pos, target_entity);
                    pos.0 = actor_pos;
                }
            }
            // attack
            else {
                apply_damage_event_writer.send(AttackAction {
                    actor: action.actor,
                    target: target_entity,
                    damage: 1,
                });
            }
        }
        // empty, move
        else {
            if let Ok(mut actor_pos) = pos_query.get_mut(action.actor) {
                map_figure_grid.reset(actor_pos.0);
                map_figure_grid.set(action.target_pos, action.actor);
                actor_pos.0 = action.target_pos;
            }

            if actor_is_player && action.target_pos == amulet_res.0 {
                next_game_state.set(GameState::Victory);
            }
        }
    }
}
