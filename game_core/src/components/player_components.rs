use bevy::prelude::*;

pub const MONSTER_TABLE: &[(&str, usize, i32)] = &[
    ("Ettin", 'E' as usize, 4),
    ("Ogre", 'O' as usize, 3),
    ("Orc", 'o' as usize, 2),
    ("Goblin", 'g' as usize, 1),
];

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster(pub usize);

#[derive(Component)]
pub struct Name(pub String);
