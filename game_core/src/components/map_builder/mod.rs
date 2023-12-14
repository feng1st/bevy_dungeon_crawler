use bevy::prelude::*;

use crate::prelude::*;

pub use dungeon::*;
pub use empty::*;

mod dungeon;
mod empty;

pub trait MapBuilder {
    fn build(
        &self,
        bound: IVec2,
    ) -> (
        MapTileGrid,
        MapFigureGrid,
        PlayerStart,
        AmuletStart,
        MonsterSpawns,
    );
}
