use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const MONSTER_NUM: usize = 19;

pub struct DrunkardMapBuilder;

impl MapBuilder for DrunkardMapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle {
        let mut rng = rand::thread_rng();

        let mut map_tile_grid = MapTileGrid::fill(bound, TileType::Wall);
        drunkard(
            &mut map_tile_grid,
            bound,
            IVec2::new(bound.x / 2, bound.y / 2),
            400,
            &mut rng,
        );
        while map_tile_grid.iter(TileType::Floor).count() < (bound.x * bound.y) as usize / 3 {
            let start = find_drunkard_start(&map_tile_grid, &mut rng);
            drunkard(&mut map_tile_grid, bound, start, 100, &mut rng);
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
            if map_tile_grid.can_enter(pos) {
                monster_spawns.push(pos);
            }
        }

        MapBundle {
            map_tile_grid,
            map_visibility_grid: MapVisibilityGrid::new(bound),
            map_figure_grid: MapFigureGrid::new(bound),
            player_start: PlayerStart(player_start),
            amulet_start: AmuletStart(amulet_start),
            monster_spawns: MonsterSpawns(monster_spawns),
        }
    }
}

fn drunkard(
    map_tile_grid: &mut MapTileGrid,
    bound: IVec2,
    start: IVec2,
    drunkard_move_num: usize,
    rng: &mut ThreadRng,
) {
    let mut drunkard_pos = start;
    for _ in 0..drunkard_move_num {
        let dir = match rng.gen_range(0..4) {
            0 => IVec2::new(1, 0),
            1 => IVec2::new(-1, 0),
            2 => IVec2::new(0, 1),
            _ => IVec2::new(0, -1),
        };
        drunkard_pos += dir;
        if drunkard_pos.x < 1 || drunkard_pos.x >= bound.x - 1 {
            drunkard_pos.x = start.x;
        }
        if drunkard_pos.y < 1 || drunkard_pos.y >= bound.y - 1 {
            drunkard_pos.y = start.y;
        }
        map_tile_grid.set(drunkard_pos, TileType::Floor);
    }
}

fn find_drunkard_start(map_tile_grid: &MapTileGrid, rng: &mut ThreadRng) -> IVec2 {
    loop {
        let start = IVec2::new(
            rng.gen_range(0..map_tile_grid.bound.x),
            rng.gen_range(0..map_tile_grid.bound.y),
        );
        if map_tile_grid.can_enter(start) {
            return start;
        }
    }
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
