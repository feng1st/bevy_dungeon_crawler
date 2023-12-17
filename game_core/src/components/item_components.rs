use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ItemTrait {
    ProvidesHealing { amount: i32 },
    ProvidesDungeonMap,
    AmuletOfYala,
}

pub const ITEM_TABLE: &[(&str, usize, &[ItemTrait])] = &[
    ("Amulet of Yala", '|' as usize, &[ItemTrait::AmuletOfYala]),
    (
        "Healing Potion",
        '!' as usize,
        &[ItemTrait::ProvidesHealing { amount: 6 }],
    ),
    (
        "Dungeon Map",
        '{' as usize,
        &[ItemTrait::ProvidesDungeonMap],
    ),
];

#[derive(Component)]
pub struct Item(pub usize);

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

// TODO: This function should belong to systems, not components.
pub fn spawn_item(commands: &mut Commands, item_id: usize, pos: IVec2) {
    let (name, _, item_traits) = ITEM_TABLE[item_id];

    let mut entity_commands = commands.spawn((Item(item_id), Name(name.to_string()), MapPos(pos)));

    add_item_trait(&mut entity_commands, item_traits);
}

fn add_item_trait(entity_commands: &mut EntityCommands, item_traits: &[ItemTrait]) {
    for &item_trait in item_traits {
        match item_trait {
            ItemTrait::ProvidesHealing { amount } => {
                entity_commands.insert(ProvidesHealing { amount });
            }
            ItemTrait::ProvidesDungeonMap => {
                entity_commands.insert(ProvidesDungeonMap);
            }
            ItemTrait::AmuletOfYala => {
                entity_commands.insert(AmuletOfYala);
            }
        }
    }
}
