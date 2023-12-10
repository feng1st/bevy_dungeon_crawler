use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn monster_chase_player(
    mut move_figure_event_writer: EventWriter<MoveAction>,
    query: Query<(Entity, &MapPos), (With<Monster>, With<ChasingMind>)>,
    map_query: Query<(&MapTileGrid, &MapFigureGrid)>,
    player_query: Query<&MapPos, (With<Player>, Without<Monster>)>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        if let Ok((map_tile_grid, map_figure_grid)) = map_query.get_single() {
            for (entity, map_pos) in &query {
                if let Some(path) = PathFinder::find_path(
                    map_tile_grid,
                    Some(map_figure_grid),
                    map_pos.0,
                    player_pos.0,
                ) {
                    if path.len() > 1 {
                        move_figure_event_writer.send(MoveAction {
                            actor: entity,
                            target_pos: path[1],
                        });
                    }
                }
            }
        }
    }
}
