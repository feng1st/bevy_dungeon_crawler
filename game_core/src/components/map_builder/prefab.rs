use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const FORTRESS: &str = "
............
...######...
...#....#...
...#.M..#...
.###....###.
..M......M..
.###....###.
...#....#...
...#....#...
...######...
............
";

pub fn try_apply_prefab(map_bundle: &mut MapBundle, n: i32) {
    let mut rng = rand::thread_rng();

    let prefab_size = probe_prefab_size(FORTRESS);
    for _ in 0..n {
        let x = rng.gen_range(1..map_bundle.map_tile_grid.bound.x - prefab_size.x - 1);
        let y = rng.gen_range(1..map_bundle.map_tile_grid.bound.y - prefab_size.y - 1);
        if can_apply_prefab(map_bundle, FORTRESS, x, y) {
            apply_prefab(map_bundle, FORTRESS, x, y);
            break;
        }
    }
}

fn probe_prefab_size(prefab: &str) -> IVec2 {
    let mut prefab_x = 0;
    let mut prefab_y = 0;
    let mut max_prefab_x = 0;
    for c in prefab.chars() {
        match c {
            '\n' | '\r' => {
                if prefab_x > 0 {
                    if prefab_x > max_prefab_x {
                        max_prefab_x = prefab_x;
                    }

                    prefab_y += 1;
                    prefab_x = 0;
                }
            }
            '#' | '.' | 'M' => {
                prefab_x += 1;
            }
            _ => (),
        }
    }
    IVec2::new(max_prefab_x, prefab_y)
}

fn can_apply_prefab(map_bundle: &MapBundle, prefab: &str, x: i32, y: i32) -> bool {
    let mut prefab_x = 0;
    let mut prefab_y = 0;
    for c in prefab.chars() {
        let pos = IVec2::new(x + prefab_x, y + prefab_y);
        match c {
            '\n' | '\r' => {
                if prefab_x > 0 {
                    prefab_y += 1;
                    prefab_x = 0;
                }
            }
            '#' | '.' | 'M' => {
                if !map_bundle.map_tile_grid.can_enter(pos) {
                    return false;
                }
                if pos == map_bundle.player_start.0 || pos == map_bundle.amulet_start.0 {
                    return false;
                }
                if map_bundle.monster_spawns.0.contains(&pos) {
                    return false;
                }
                prefab_x += 1;
            }
            _ => (),
        }
    }
    true
}

fn apply_prefab(map_bundle: &mut MapBundle, prefab: &str, x: i32, y: i32) {
    let mut prefab_x = 0;
    let mut prefab_y = 0;
    for c in prefab.chars() {
        let pos = IVec2::new(x + prefab_x, y + prefab_y);
        match c {
            '\n' | '\r' => {
                if prefab_x > 0 {
                    prefab_y += 1;
                    prefab_x = 0;
                }
            }
            '#' => {
                map_bundle.map_tile_grid.set(pos, TileType::Wall);
                prefab_x += 1;
            }
            '.' => {
                map_bundle.map_tile_grid.set(pos, TileType::Floor);
                prefab_x += 1;
            }
            'M' => {
                map_bundle.map_tile_grid.set(pos, TileType::Floor);
                map_bundle.monster_spawns.0.push(pos);
                prefab_x += 1;
            }
            _ => (),
        }
    }
}
