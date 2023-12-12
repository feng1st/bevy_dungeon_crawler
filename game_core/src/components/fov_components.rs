use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<IVec2>,
    pub radius: i32,
}

impl FieldOfView {
    #[must_use]
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
        }
    }
}

#[derive(Component, Default)]
pub struct LastFieldOfView(pub HashSet<IVec2>);
