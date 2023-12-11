use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(
    mut commands: Commands,
    map_query: Query<&MapTileGrid>,
    player_query: Query<&MapPos, With<Player>>,
    mut amulet_res: ResMut<AmuletOfYalaPos>,
) {
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };
    let Ok(map_tile_grid) = map_query.get_single() else {
        return;
    };

    if let Some(amulet_pos) = map_tile_grid
        .iter(TileType::Floor)
        .filter_map(|pos| {
            PathFinder::find_cost(map_tile_grid, None, pos, player_pos.0).map(|cost| (pos, cost))
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(pos, _)| pos)
    {
        amulet_res.0 = amulet_pos;
        commands.spawn((Item, AmuletOfYala, MapPos(amulet_pos)));
    }
}
