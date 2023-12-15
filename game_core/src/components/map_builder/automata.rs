use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const MONSTER_NUM: usize = 19;

pub struct AutomataMapBuilder;

impl MapBuilder for AutomataMapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle {
        let mut rng = rand::thread_rng();

        let mut map_tile_grid_origin = MapTileGrid::fill(bound, TileType::Floor);
        for x in 0..bound.x {
            for y in 0..bound.y {
                if rng.gen_range(0..100) < 45 {
                    map_tile_grid_origin.set(IVec2::new(x, y), TileType::Wall);
                }
            }
        }

        let mut map_tile_grid = MapTileGrid::fill(bound, TileType::Wall);
        for x in 1..bound.x - 1 {
            for y in 1..bound.y - 1 {
                let pos = IVec2::new(x, y);
                let neighbors = count_neighbors(&map_tile_grid_origin, pos);
                if neighbors > 0 && neighbors < 5 {
                    map_tile_grid.set(pos, TileType::Floor);
                }
            }
        }

        let player_start = find_start(&map_tile_grid);

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
            if map_tile_grid.can_enter(pos)
                && PathFinder::find_path(&map_tile_grid, None, pos, player_start).is_some()
            {
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

fn count_neighbors(map_tile_grid: &MapTileGrid, pos: IVec2) -> usize {
    let mut count = 0;
    for x in pos.x - 1..=pos.x + 1 {
        for y in pos.y - 1..=pos.y + 1 {
            if map_tile_grid.get(IVec2::new(x, y)) == TileType::Wall {
                count += 1;
            }
        }
    }
    count
}

fn find_start(map_tile_grid: &MapTileGrid) -> IVec2 {
    let center = map_tile_grid.bound / 2;
    map_tile_grid
        .iter(TileType::Floor)
        .min_by(|a, b| {
            a.distance_squared(center)
                .partial_cmp(&b.distance_squared(center))
                .unwrap()
        })
        .unwrap()
}
