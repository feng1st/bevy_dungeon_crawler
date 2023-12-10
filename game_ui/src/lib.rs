#![warn(clippy::all, clippy::pedantic)]

pub mod prelude {
    pub use crate::components::*;
}

mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;
use prelude::*;
use systems::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_plugins(TilemapPlugin)
            .init_resource::<CursorPosition>()
            .add_systems(Startup, load_assets.in_set(UiSystemSet::StartupLoadAssets))
            .add_systems(
                Startup,
                (spawn_tooltip, spawn_camera).in_set(UiSystemSet::StartupSetupGlobalUi),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                build_in_game_ui.in_set(UiSystemSet::OnInGameSetupUi),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    system_pre_cleanup::<TilePos>,
                    system_pre_cleanup::<TileStorage>,
                )
                    .in_set(UiSystemSet::OnInGameCleanupSprites),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_tilemap, spawn_player_sprite, spawn_monster_sprites)
                    .in_set(UiSystemSet::OnInGameSetupSprites),
            )
            .add_systems(
                OnExit(GameState::InGame),
                (system_cleanup::<RootNode>, hide_tooltip).in_set(UiSystemSet::OnInGameCleanupUi),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                build_game_over_ui.in_set(UiSystemSet::OnGameOverSetupUi),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (system_cleanup::<RootNode>, hide_tooltip).in_set(UiSystemSet::OnGameOverCleanupUi),
            )
            .add_systems(
                Update,
                update_in_game_ui_player_health.in_set(UiSystemSet::InGameUpdateUi),
            )
            .add_systems(
                Update,
                (
                    map_pos_to_trans,
                    update_camera,
                    update_cursor_position,
                    update_in_game_ui_tooltip,
                )
                    .chain()
                    .in_set(UiSystemSet::InGameUpdateSprites),
            );
    }
}
