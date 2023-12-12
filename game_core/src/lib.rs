pub mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;

    pub use crate::{GameState, GameTurn};

    pub use crate::UiSystemSet;
}

mod components;
mod systems;

use bevy::prelude::*;
use prelude::*;

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    Victory,
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
    StartupLoadAssets,
    StartupSetupGlobalUi,

    OnInGameSetupUi,
    OnInGameCleanupUi,
    OnInGameSetupSprites,
    OnInGameCleanupSprites,

    OnGameOverSetupUi,
    OnGameOverCleanupUi,

    OnVictorySetupUi,
    OnVictoryCleanupUi,

    InGameUpdateUi,
    InGameUpdateSprites,
}

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CoreSystemSet {
    OnInGameSetupData,
    OnInGameCleanupData,

    InGamePlayerInput,
    InGameMonsterThink,
    InGameUpdateData,
    InGameEndTurn,

    GameOverPlayerInput,

    VictoryPlayerInput,
}

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        GameCorePlugin::register_states(app);
        GameCorePlugin::register_events(app);
        GameCorePlugin::init_resources(app);
        GameCorePlugin::config_system_sets(app);
        GameCorePlugin::add_core_game_systems(app);
    }
}

impl GameCorePlugin {
    fn register_states(app: &mut App) {
        app.add_state::<GameState>().add_state::<GameTurn>();
    }

    fn register_events(app: &mut App) {
        app.add_event::<MoveAction>()
            .add_event::<AttackAction>()
            .add_event::<RestAction>();
    }

    fn init_resources(app: &mut App) {
        app.init_resource::<AmuletOfYalaPos>();
    }

    fn config_system_sets(app: &mut App) {
        app.configure_sets(
            Startup,
            (
                UiSystemSet::StartupLoadAssets,
                UiSystemSet::StartupSetupGlobalUi,
            ),
        )
        .add_systems(
            Startup,
            apply_deferred
                .after(UiSystemSet::StartupLoadAssets)
                .before(UiSystemSet::StartupSetupGlobalUi),
        )
        .configure_sets(
            OnEnter(GameState::InGame),
            (
                UiSystemSet::OnInGameSetupUi,
                UiSystemSet::OnInGameCleanupSprites,
                CoreSystemSet::OnInGameCleanupData,
                CoreSystemSet::OnInGameSetupData,
                UiSystemSet::OnInGameSetupSprites,
            ),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                apply_deferred
                    .after(UiSystemSet::OnInGameCleanupSprites)
                    .before(CoreSystemSet::OnInGameCleanupData),
                apply_deferred
                    .after(CoreSystemSet::OnInGameCleanupData)
                    .before(CoreSystemSet::OnInGameSetupData),
                apply_deferred
                    .after(CoreSystemSet::OnInGameSetupData)
                    .before(UiSystemSet::OnInGameSetupSprites),
            ),
        )
        .configure_sets(OnExit(GameState::InGame), UiSystemSet::OnInGameCleanupUi)
        .configure_sets(OnEnter(GameState::GameOver), UiSystemSet::OnGameOverSetupUi)
        .configure_sets(
            OnExit(GameState::GameOver),
            UiSystemSet::OnGameOverCleanupUi,
        )
        .configure_sets(OnEnter(GameState::Victory), UiSystemSet::OnVictorySetupUi)
        .configure_sets(OnExit(GameState::Victory), UiSystemSet::OnVictoryCleanupUi)
        .configure_sets(
            Update,
            (
                CoreSystemSet::InGamePlayerInput.run_if(in_state(GameTurn::Player)),
                CoreSystemSet::InGameMonsterThink.run_if(in_state(GameTurn::Monster)),
                CoreSystemSet::InGameUpdateData,
                UiSystemSet::InGameUpdateUi,
                UiSystemSet::InGameUpdateSprites,
                CoreSystemSet::InGameEndTurn,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (
                apply_deferred
                    .after(CoreSystemSet::InGamePlayerInput)
                    .after(CoreSystemSet::InGameMonsterThink)
                    .before(CoreSystemSet::InGameUpdateData),
                apply_deferred
                    .after(CoreSystemSet::InGameUpdateData)
                    .before(UiSystemSet::InGameUpdateUi)
                    .before(UiSystemSet::InGameUpdateSprites),
                apply_deferred
                    .after(UiSystemSet::InGameUpdateUi)
                    .after(UiSystemSet::InGameUpdateSprites)
                    .before(CoreSystemSet::InGameEndTurn),
            ),
        )
        .configure_sets(
            Update,
            CoreSystemSet::GameOverPlayerInput.run_if(in_state(GameState::GameOver)),
        )
        .configure_sets(
            Update,
            CoreSystemSet::VictoryPlayerInput.run_if(in_state(GameState::Victory)),
        );
    }

    fn add_core_game_systems(app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (
                system_cleanup::<Item>,
                system_cleanup::<Monster>,
                system_cleanup::<Player>,
                system_cleanup::<Map>,
            )
                .in_set(CoreSystemSet::OnInGameCleanupData),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                build_map,
                spawn_player,
                spawn_monsters,
                spawn_amulet_of_yala,
                apply_deferred
                    .after(build_map)
                    .before(spawn_player)
                    .before(spawn_monsters),
                apply_deferred
                    .after(spawn_player)
                    .before(spawn_amulet_of_yala),
            )
                .in_set(CoreSystemSet::OnInGameSetupData),
        )
        .add_systems(
            Update,
            player_input.in_set(CoreSystemSet::InGamePlayerInput),
        )
        .add_systems(
            Update,
            (monster_think, monster_wander, monster_chase_player)
                .in_set(CoreSystemSet::InGameMonsterThink),
        )
        .add_systems(
            Update,
            (
                move_figure,
                take_damage.after(move_figure),
                take_rest.after(take_damage),
                calc_fov.after(move_figure),
            )
                .in_set(CoreSystemSet::InGameUpdateData),
        )
        .add_systems(Update, end_turn.in_set(CoreSystemSet::InGameEndTurn))
        .add_systems(
            Update,
            player_input_end_game.in_set(CoreSystemSet::GameOverPlayerInput),
        )
        .add_systems(
            Update,
            player_input_end_game.in_set(CoreSystemSet::VictoryPlayerInput),
        )
        .add_systems(Last, system_cleanup::<Delete>);
    }
}
