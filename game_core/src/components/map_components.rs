use bevy::prelude::*;
use bevy_fov::VisibilityMap;

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

pub trait MapGrid<T> {
    fn bound(&self) -> IVec2;

    fn get_element(&self, index: usize) -> T;

    fn set_element(&mut self, index: usize, value: T);

    fn get_at(&self, pos: IVec2) -> Option<T> {
        if self.in_bounds(pos) {
            Some(self.get_element(self.pos_to_index(pos)))
        } else {
            None
        }
    }

    fn set_at(&mut self, pos: IVec2, value: T) {
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            self.set_element(index, value);
        }
    }

    fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.bound().x && pos.y >= 0 && pos.y < self.bound().y
    }

    fn pos_to_index(&self, pos: IVec2) -> usize {
        (self.bound().x * pos.y + pos.x) as usize
    }

    fn index_to_pos(&self, index: usize) -> IVec2 {
        IVec2::new(index as i32 % self.bound().x, index as i32 / self.bound().x)
    }
}

#[derive(Component)]
pub struct MapTileGrid {
    pub bound: IVec2,
    pub tiles: Vec<TileType>,
}

impl MapGrid<TileType> for MapTileGrid {
    fn bound(&self) -> IVec2 {
        self.bound
    }

    fn get_element(&self, index: usize) -> TileType {
        self.tiles[index]
    }

    fn set_element(&mut self, index: usize, value: TileType) {
        self.tiles[index] = value;
    }
}

impl VisibilityMap for MapTileGrid {
    fn is_opaque(&self, p: IVec2) -> bool {
        self.get(p) != TileType::Floor
    }

    fn is_in_bounds(&self, p: IVec2) -> bool {
        self.in_bounds(p)
    }

    fn dist(&self, a: IVec2, b: IVec2) -> f32 {
        Vec2::new(a.x as f32, a.y as f32).distance(Vec2::new(b.x as f32, b.y as f32))
    }
}

impl MapTileGrid {
    #[must_use]
    pub fn fill(bound: IVec2, tile_type: TileType) -> Self {
        Self {
            bound,
            tiles: vec![tile_type; (bound.x * bound.y) as usize],
        }
    }

    #[must_use]
    pub fn can_enter(&self, pos: IVec2) -> bool {
        self.get(pos) == TileType::Floor
    }

    #[must_use]
    pub fn get(&self, pos: IVec2) -> TileType {
        self.get_at(pos).unwrap_or(TileType::Void)
    }

    pub fn set(&mut self, pos: IVec2, value: TileType) {
        self.set_at(pos, value);
    }

    pub fn iter(&self, tile_type: TileType) -> impl Iterator<Item = IVec2> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(index, &tile)| {
                if tile == tile_type {
                    Some(self.index_to_pos(index))
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

impl MapGrid<Entity> for MapFigureGrid {
    fn bound(&self) -> IVec2 {
        self.bound
    }

    fn get_element(&self, index: usize) -> Entity {
        self.figures[index]
    }

    fn set_element(&mut self, index: usize, value: Entity) {
        self.figures[index] = value;
    }
}

impl MapFigureGrid {
    #[must_use]
    pub fn new(bound: IVec2) -> Self {
        Self {
            bound,
            figures: vec![Entity::PLACEHOLDER; (bound.x * bound.y) as usize],
        }
    }

    #[must_use]
    pub fn get(&self, pos: IVec2) -> Entity {
        self.get_at(pos).unwrap_or(Entity::PLACEHOLDER)
    }

    pub fn set(&mut self, pos: IVec2, value: Entity) {
        self.set_at(pos, value);
    }

    pub fn reset(&mut self, pos: IVec2) {
        self.set_at(pos, Entity::PLACEHOLDER);
    }
}

#[derive(Component)]
pub struct PlayerStart(pub IVec2);

#[derive(Component)]
pub struct AmuletStart(pub IVec2);

#[derive(Component)]
pub struct MonsterSpawns(pub Vec<IVec2>);

#[derive(Bundle)]
pub struct MapBundle {
    pub map_tile_grid: MapTileGrid,
    pub map_figure_grid: MapFigureGrid,
    pub player_start: PlayerStart,
    pub amulet_start: AmuletStart,
    pub monster_spawns: MonsterSpawns,
}
