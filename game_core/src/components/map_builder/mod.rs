use bevy::prelude::*;

use crate::prelude::*;

pub use automata::*;
pub use drunkard::*;
pub use dungeon::*;
pub use empty::*;
pub use prefab::*;

mod automata;
mod drunkard;
mod dungeon;
mod empty;
mod prefab;

pub trait MapBuilder {
    fn build(&self, bound: IVec2) -> MapBundle;
}
