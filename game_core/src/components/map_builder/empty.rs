use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const ROOM_NUM: usize = 20;

pub struct EmptyMapBuilder;

impl MapBuilder for EmptyMapBuilder {
    fn build(
        &self,
        bound: IVec2,
    ) -> (
        MapTileGrid,
        MapFigureGrid,
        PlayerStart,
        AmuletStart,
        MonsterSpawns,
    ) {
        let mut rng = rand::thread_rng();

        let map_tile_grid = MapTileGrid::fill(bound, TileType::Floor);

        let player_start = IVec2::new(bound.x / 2, bound.y / 2);

        let amulet_start = map_tile_grid
            .iter(TileType::Floor)
            .filter_map(|pos| {
                PathFinder::find_cost(&map_tile_grid, None, pos, player_start)
                    .map(|cost| (pos, cost))
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(pos, _)| pos)
            .unwrap();

        let monster_spawns = (1..ROOM_NUM)
            .map(|_| IVec2::new(rng.gen_range(0..bound.x), rng.gen_range(0..bound.y)))
            .collect();

        (
            map_tile_grid,
            MapFigureGrid::new(bound),
            PlayerStart(player_start),
            AmuletStart(amulet_start),
            MonsterSpawns(monster_spawns),
        )
    }
}
