use crate::prelude::*;

pub struct DungeonTheme;

impl MapTheme for DungeonTheme {
    fn get_texture_index(&self, tile_type: TileType) -> u32 {
        match tile_type {
            TileType::Floor => '.' as u32,
            TileType::Wall => '#' as u32,
            TileType::Void => 0,
        }
    }
}

pub struct ForestTheme;

impl MapTheme for ForestTheme {
    fn get_texture_index(&self, tile_type: TileType) -> u32 {
        match tile_type {
            TileType::Floor => ';' as u32,
            TileType::Wall => '"' as u32,
            TileType::Void => 0,
        }
    }
}
