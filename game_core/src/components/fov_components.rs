use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<IVec2>,
    pub radius: i32,
    pub is_dirty: bool,
}
