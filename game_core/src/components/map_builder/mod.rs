use bevy::prelude::*;

use crate::prelude::*;

pub use automata::*;
pub use drunkard::*;
pub use dungeon::*;
pub use empty::*;

mod automata;
mod drunkard;
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
