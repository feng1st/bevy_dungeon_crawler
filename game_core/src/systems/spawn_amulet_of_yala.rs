use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(
    mut commands: Commands,
    map_query: Query<&MapTileGrid>,
    player_query: Query<&MapPos, With<Player>>,
    mut amulet_res: ResMut<AmuletOfYalaPos>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        if let Ok(map_tile_grid) = map_query.get_single() {
            if let Some(amulet_pos) = map_tile_grid
                .iter(TileType::Floor)
                .filter_map(|pos| {
                    PathFinder::find_path_with_cost(map_tile_grid, None, pos, player_pos.0)
                })
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .filter(|(path, _)| path.len() > 0)
                .map(|(path, _)| path[0])
            {
                amulet_res.0 = amulet_pos;
                commands.spawn((Item, AmuletOfYala, MapPos(amulet_pos)));
            }
        }
    }
}