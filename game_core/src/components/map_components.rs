use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Void,
    Floor,
    Wall,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct MapPos(pub IVec2);

#[derive(Component)]
pub struct Map;

pub trait MapBound {
    fn bound(&self) -> &IVec2;

    fn to_index(&self, pos: IVec2) -> usize {
        (self.bound().x * pos.y + pos.x) as usize
    }

    fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.bound().x && pos.y >= 0 && pos.y < self.bound().y
    }
}

#[derive(Component)]
pub struct MapTileGrid {
    pub bound: IVec2,
    pub tiles: Vec<TileType>,
}

impl MapBound for MapTileGrid {
    fn bound(&self) -> &IVec2 {
        &self.bound
    }
}

impl MapTileGrid {
    pub fn fill(bound: IVec2, tile_type: TileType) -> Self {
        Self {
            bound,
            tiles: vec![tile_type; (bound.x * bound.y) as usize],
        }
    }

    pub fn can_enter(&self, pos: IVec2) -> bool {
        self.in_bounds(pos) && self.tiles[self.to_index(pos)] == TileType::Floor
    }

    pub fn get(&self, pos: IVec2) -> TileType {
        if self.in_bounds(pos) {
            self.tiles[self.to_index(pos)]
        } else {
            TileType::Void
        }
    }

    pub fn set(&mut self, pos: IVec2, tile_type: TileType) {
        if self.in_bounds(pos) {
            let index = self.to_index(pos);
            self.tiles[index] = tile_type;
        }
    }

    pub fn iter(&self, tile_type: TileType) -> impl Iterator<Item = IVec2> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(index, &tile)| {
                if tile == tile_type {
                    let x = index as i32 % self.bound.x;
                    let y = index as i32 / self.bound.x;
                    Some(IVec2::new(x, y))
                } else {
                    None
                }
            })
    }
}

#[derive(Component)]
pub struct MapFigureGrid {
    pub bound: IVec2,
    pub figures: Vec<Entity>,
}

impl MapBound for MapFigureGrid {
    fn bound(&self) -> &IVec2 {
        &self.bound
    }
}

impl MapFigureGrid {
    pub fn new(bound: IVec2) -> Self {
        Self {
            bound,
            figures: vec![Entity::PLACEHOLDER; (bound.x * bound.y) as usize],
        }
    }

    pub fn get(&self, pos: IVec2) -> Entity {
        if self.in_bounds(pos) {
            self.figures[self.to_index(pos)]
        } else {
            Entity::PLACEHOLDER
        }
    }

    pub fn set(&mut self, pos: IVec2, entity: Entity) {
        if self.in_bounds(pos) {
            let index = self.to_index(pos);
            self.figures[index] = entity;
        }
    }

    pub fn reset(&mut self, pos: IVec2) {
        if self.in_bounds(pos) {
            let index = self.to_index(pos);
            self.figures[index] = Entity::PLACEHOLDER;
        }
    }
}

#[derive(Component)]
pub struct MapRoomList(pub Vec<IRect>);
