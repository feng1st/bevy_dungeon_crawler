pub mod prelude {
    pub use crate::components::*;
}

mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;
use prelude::*;
use systems::prelude::*;

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
                (
                    spawn_tilemap,
                    spawn_player_sprite,
                    spawn_monster_sprites,
                    spawn_amulet_of_yala_sprite,
                )
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
                OnEnter(GameState::Victory),
                build_victory_ui.in_set(UiSystemSet::OnVictorySetupUi),
            )
            .add_systems(
                OnExit(GameState::Victory),
                (system_cleanup::<RootNode>, hide_tooltip).in_set(UiSystemSet::OnVictoryCleanupUi),
            )
            .add_systems(
                Update,
                update_in_game_ui_player_health.in_set(UiSystemSet::InGameUpdateUi),
            )
            .add_systems(
                Update,
                (
                    update_tilemap_fov,
                    update_monster_fov,
                    map_pos_to_trans,
                    update_camera.after(map_pos_to_trans),
                    update_cursor_position.after(update_camera),
                    update_in_game_ui_tooltip.after(update_cursor_position),
                )
                    .in_set(UiSystemSet::InGameUpdateSprites),
            );
    }
}
