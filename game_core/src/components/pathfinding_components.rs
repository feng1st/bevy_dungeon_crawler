use crate::prelude::*;
use bevy::prelude::*;
use pathfinding::prelude::*;

pub struct PathFinder;

impl PathFinder {
    pub fn find_path(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        end: IVec2,
    ) -> Option<Vec<IVec2>> {
        PathFinder::find_path_with_cost(map_tile_grid, map_figure_grid, start, end)
            .map(|(path, _)| path)
    }

    pub fn find_path_with_cost(
        map_tile_grid: &MapTileGrid,
        map_figure_grid: Option<&MapFigureGrid>,
        start: IVec2,
        end: IVec2,
    ) -> Option<(Vec<IVec2>, u32)> {
        astar(
            &start,
            |pos| PathFinder::neighbors(map_tile_grid, map_figure_grid, *pos),
            |pos| PathFinder::manhattan_distance(*pos, end),
            |pos| *pos == end,
        )
    }

    fn neighbors(
        tile_grid: &MapTileGrid,
        figure_grid: Option<&MapFigureGrid>,
        pos: IVec2,
    ) -> Vec<(IVec2, u32)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions {
            let neighbor_pos = pos + IVec2::new(dx, dy);
            if tile_grid.can_enter(neighbor_pos) {
                if figure_grid.is_some_and(|f| f.get(neighbor_pos) != Entity::PLACEHOLDER) {
                    neighbors.push((neighbor_pos, 8));
                } else {
                    neighbors.push((neighbor_pos, 1));
                }
            }
        }

        neighbors
    }

    fn manhattan_distance(pos: IVec2, other: IVec2) -> u32 {
        ((pos.x - other.x).abs() + (pos.y - other.y).abs()) as u32
    }
}
