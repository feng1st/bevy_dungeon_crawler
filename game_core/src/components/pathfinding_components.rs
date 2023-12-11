use crate::prelude::*;
use bevy::prelude::*;
use pathfinding::prelude::*;

pub struct PathFinder;

impl PathFinder {
    pub fn find_next_pos(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        goal: IVec2,
    ) -> Option<IVec2> {
        PathFinder::find_path(map_tile_grid, map_figure_grid, start, goal)
            .filter(|path| path.len() > 1)
            .map(|path| path[1])
    }

    pub fn find_path(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        goal: IVec2,
    ) -> Option<Vec<IVec2>> {
        PathFinder::find_path_and_cost(map_tile_grid, map_figure_grid, start, goal)
            .map(|(path, _)| path)
    }

    pub fn find_cost(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        goal: IVec2,
    ) -> Option<u32> {
        PathFinder::find_path_and_cost(map_tile_grid, map_figure_grid, start, goal)
            .map(|(_, cost)| cost)
    }

    pub fn find_path_and_cost(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        goal: IVec2,
    ) -> Option<(Vec<IVec2>, u32)> {
        astar(
            &start,
            |pos| PathFinder::neighbors(map_tile_grid, map_figure_grid, *pos),
            |pos| PathFinder::manhattan_distance(*pos, goal),
            |pos| *pos == goal,
        )
    }

    fn neighbors(
        tile_grid: &MapTileGrid,
        figure_grid: Option<&MapFigureGrid>,
        pos: IVec2,
    ) -> Vec<(IVec2, u32)> {
        let mut neighbors = Vec::new();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor_pos = pos + IVec2::new(dx, dy);
            if !tile_grid.can_enter(neighbor_pos) {
                continue;
            }
            neighbors.push((
                neighbor_pos,
                if figure_grid
                    .is_some_and(|figure_grid| figure_grid.get(neighbor_pos) != Entity::PLACEHOLDER)
                {
                    8
                } else {
                    1
                },
            ));
        }
        neighbors
    }

    fn manhattan_distance(pos: IVec2, other: IVec2) -> u32 {
        ((pos.x - other.x).abs() + (pos.y - other.y).abs()) as u32
    }
}
