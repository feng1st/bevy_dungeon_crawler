#![warn(clippy::all, clippy::pedantic)]

pub mod prelude {
    pub use crate::components::*;

    pub use crate::{GameState, GameTurn};

    pub use crate::UiSystemSet;
}

mod components;
mod systems;

use bevy::prelude::*;
use prelude::*;
use systems::*;

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    GameOver,
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameTurn {
    #[default]
    Player,
    Monster,
}

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiSystemSet {
    LoadStaticAssets,
    SetupGlobalUi,
    SetupInGameUi,
    SetupInGameSprites,
    CleanInGameUi,
    SetupGameOverUi,
    UpdateUi,
}

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CoreSystemSet {
    SetupInGameData,
}

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<GameTurn>()
            .add_event::<MoveAction>()
            .add_event::<AttackAction>()
            .add_event::<RestAction>()
            .configure_sets(PreStartup, UiSystemSet::LoadStaticAssets)
            .configure_sets(Startup, UiSystemSet::SetupGlobalUi)
            .configure_sets(OnEnter(GameState::InGame), UiSystemSet::SetupInGameUi)
            .configure_sets(
                OnEnter(GameState::InGame),
                (
                    CoreSystemSet::SetupInGameData,
                    UiSystemSet::SetupInGameSprites,
                )
                    .chain(),
            )
            .configure_sets(OnExit(GameState::InGame), UiSystemSet::CleanInGameUi)
            .configure_sets(OnEnter(GameState::GameOver), UiSystemSet::SetupGameOverUi)
            .configure_sets(PostUpdate, UiSystemSet::UpdateUi)
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    build_map,
                    apply_deferred,
                    (spawn_player, spawn_monsters),
                    apply_deferred,
                )
                    .chain()
                    .in_set(CoreSystemSet::SetupInGameData),
            )
            .add_systems(
                PreUpdate,
                (
                    player_input
                        .run_if(in_state(GameState::InGame))
                        .run_if(in_state(GameTurn::Player)),
                    (monster_think, monster_wander, monster_chase_player)
                        .run_if(in_state(GameState::InGame))
                        .run_if(in_state(GameTurn::Monster)),
                ),
            )
            .add_systems(
                Update,
                (move_figure, take_rest, take_damage)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(Last, end_turn.run_if(in_state(GameState::InGame)));
    }
}
