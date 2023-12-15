use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const MONSTER_NUM: usize = 19;

pub struct EmptyMapBuilder;

impl MapBuilder for EmptyMapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle {
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

        let mut monster_spawns = vec![];
        while monster_spawns.len() < MONSTER_NUM {
            let pos = IVec2::new(rng.gen_range(0..bound.x), rng.gen_range(0..bound.y));
            if map_tile_grid.can_enter(pos) {
                monster_spawns.push(pos);
            }
        }

        MapBundle {
            map_tile_grid,
            map_figure_grid: MapFigureGrid::new(bound),
            player_start: PlayerStart(player_start),
            amulet_start: AmuletStart(amulet_start),
            monster_spawns: MonsterSpawns(monster_spawns),
        }
    }
}
