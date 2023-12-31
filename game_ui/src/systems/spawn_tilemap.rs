use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

pub fn spawn_tilemap(
    mut commands: Commands,
    query: Query<(&MapTileGrid, &MapThemeComponent)>,
    main_texture: Res<MainTexture>,
) {
    if let Ok((map_tile_grid, map_theme)) = query.get_single() {
        let tilemap_size = TilemapSize {
            x: map_tile_grid.bound.x as u32,
            y: map_tile_grid.bound.y as u32,
        };
        let mut tile_storage = TileStorage::empty(tilemap_size);
        let entity: Entity = commands.spawn_empty().id();
        for x in 0..tilemap_size.x {
            for y in 0..tilemap_size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity =
                    commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(map_theme.0.get_texture_index(
                                map_tile_grid.get(IVec2::new(x as i32, y as i32)),
                            )),
                            tilemap_id: TilemapId(entity),
                            visible: TileVisible(false),
                            ..Default::default()
                        })
                        .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let tile_size = TilemapTileSize {
            x: MAIN_TEXTURE_TILE_WIDTH,
            y: MAIN_TEXTURE_TILE_HEIGHT,
        };
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();

        commands.entity(entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(main_texture.0.clone()),
            tile_size,
            ..Default::default()
        });
    }
}
