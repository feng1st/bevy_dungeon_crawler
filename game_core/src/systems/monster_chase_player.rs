use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn monster_chase_player(
    mut move_figure_event_writer: EventWriter<MoveAction>,
    monster_query: Query<(Entity, &MapPos), (With<Monster>, With<ChasingMind>)>,
    map_query: Query<(&MapTileGrid, &MapFigureGrid)>,
    player_query: Query<&MapPos, (With<Player>, Without<Monster>)>,
) {
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };

    let Ok((map_tile_grid, map_figure_grid)) = map_query.get_single() else {
        return;
    };

    for (monster_entity, monster_pos) in &monster_query {
        let Some(target_pos) = PathFinder::find_next_pos(
            map_tile_grid,
            Some(map_figure_grid),
            monster_pos.0,
            player_pos.0,
        ) else {
            continue;
        };

        move_figure_event_writer.send(MoveAction {
            actor: monster_entity,
            target_pos,
        });
    }
}
