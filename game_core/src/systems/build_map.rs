use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

const MAP_BOUND: IVec2 = IVec2::new(80, 50);

const ROOM_NUM: usize = 20;

pub fn build_map(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let mut map_tile_grid = MapTileGrid::fill(MAP_BOUND, TileType::Wall);

    let rooms = gen_rooms(&mut rng);

    carve_rooms(&mut map_tile_grid, &rooms);
    carve_corridors(&mut map_tile_grid, &rooms, &mut rng);

    commands.spawn((
        map_tile_grid,
        MapFigureGrid::new(MAP_BOUND),
        MapRoomList(rooms),
    ));
}

fn gen_rooms(rng: &mut ThreadRng) -> Vec<IRect> {
    let mut rooms = vec![];

    while rooms.len() < ROOM_NUM {
        let room = IRect::from_center_half_size(
            IVec2::new(
                rng.gen_range(6..MAP_BOUND.x - 6),
                rng.gen_range(5..MAP_BOUND.y - 5),
            ),
            IVec2::new(rng.gen_range(2..6), rng.gen_range(2..5)),
        );

        let mut overlap = false;
        for r in &rooms {
            if !room.intersect(*r).is_empty() {
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
    for room in rooms {
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
