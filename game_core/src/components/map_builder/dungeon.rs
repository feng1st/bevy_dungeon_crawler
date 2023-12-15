use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const ROOM_NUM: usize = 20;

pub struct DungeonMapBuilder;

impl MapBuilder for DungeonMapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle {
        let mut rng = rand::thread_rng();

        let mut map_tile_grid = MapTileGrid::fill(bound, TileType::Wall);

        let rooms = gen_rooms(bound, &mut rng);

        carve_rooms(&mut map_tile_grid, &rooms);
        carve_corridors(&mut map_tile_grid, &rooms, &mut rng);

        let player_start = rooms[0].center();

        let amulet_start = map_tile_grid
            .iter(TileType::Floor)
            .filter_map(|pos| {
                PathFinder::find_cost(&map_tile_grid, None, pos, player_start)
                    .map(|cost| (pos, cost))
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(pos, _)| pos)
            .unwrap();

        let monster_spawns = rooms.iter().skip(1).map(IRect::center).collect();

        MapBundle {
            map_tile_grid,
            map_figure_grid: MapFigureGrid::new(bound),
            player_start: PlayerStart(player_start),
            amulet_start: AmuletStart(amulet_start),
            monster_spawns: MonsterSpawns(monster_spawns),
        }
    }
}

fn gen_rooms(bound: IVec2, rng: &mut ThreadRng) -> Vec<IRect> {
    let mut rooms = vec![];

    while rooms.len() < ROOM_NUM {
        let room = IRect::from_center_half_size(
            IVec2::new(rng.gen_range(6..bound.x - 6), rng.gen_range(5..bound.y - 5)),
            IVec2::new(rng.gen_range(2..6), rng.gen_range(2..5)),
        );

        let mut overlap = false;
        for &r in &rooms {
            if !room.intersect(r).is_empty() {
                overlap = true;
                break;
            }
        }

        if !overlap {
            rooms.push(room);
        }
    }
    rooms
}

fn carve_rooms(map_tile_grid: &mut MapTileGrid, rooms: &Vec<IRect>) {
    for &room in rooms {
        for x in room.min.x..room.max.x {
            for y in room.min.y..room.max.y {
                map_tile_grid.set(IVec2::new(x, y), TileType::Floor);
            }
        }
    }
}

fn carve_corridors(map_tile_grid: &mut MapTileGrid, rooms: &Vec<IRect>, rng: &mut ThreadRng) {
    let mut rs = rooms.to_owned();
    rs.sort_by(|a, b| a.center().x.cmp(&b.center().x));
    for i in 1..ROOM_NUM {
        let start = rs[i - 1].center();
        let end = rs[i].center();
        let min = start.min(end);
        let max = start.max(end);
        if rng.gen_bool(0.5) {
            (min.x..=max.x).for_each(|x| {
                map_tile_grid.set(IVec2::new(x, start.y), TileType::Floor);
            });
            (min.y..=max.y).for_each(|y| {
                map_tile_grid.set(IVec2::new(end.x, y), TileType::Floor);
            });
        } else {
            (min.y..=max.y).for_each(|y| {
                map_tile_grid.set(IVec2::new(start.x, y), TileType::Floor);
            });
            (min.x..=max.x).for_each(|x| {
                map_tile_grid.set(IVec2::new(x, end.y), TileType::Floor);
            });
        }
    }
}
