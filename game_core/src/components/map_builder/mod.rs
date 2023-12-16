use bevy::prelude::*;

use crate::prelude::*;

pub use automata::*;
pub use drunkard::*;
pub use dungeon::*;
pub use empty::*;
pub use prefab::*;
pub use themes::*;

mod automata;
mod drunkard;
mod dungeon;
mod empty;
mod prefab;
mod themes;

pub trait MapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle;
}

pub trait MapTheme: Sync + Send {
    fn get_texture_index(&self, tile_type: TileType) -> u32;
}

#[derive(Component)]
pub struct MapThemeComponent(pub Box<dyn MapTheme>);
