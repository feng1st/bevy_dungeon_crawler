#![warn(clippy::all, clippy::pedantic)]

pub mod prelude {
    pub use crate::components::*;
}

mod components;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;
use prelude::*;
use systems::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_plugins((
            TilemapPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .init_resource::<CursorPosition>()
        .add_systems(
            PreStartup,
            load_assets.in_set(UiSystemSet::LoadStaticAssets),
        )
        .add_systems(
            Startup,
            (spawn_tooltip, spawn_camera).in_set(UiSystemSet::SetupGlobalUi),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            build_in_game_ui.in_set(UiSystemSet::SetupInGameUi),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_tilemap, spawn_player_sprite, spawn_monster_sprites)
                .in_set(UiSystemSet::SetupInGameSprites),
        )
        .add_systems(
            OnExit(GameState::InGame),
            despawn_in_game_ui.in_set(UiSystemSet::CleanInGameUi),
        )
        .add_systems(
            PostUpdate,
            update_in_game_ui_player_health
                .run_if(in_state(GameState::InGame))
                .in_set(UiSystemSet::UpdateUi),
        )
        .add_systems(
            PostUpdate,
            (
                map_pos_to_trans,
                update_camera,
                update_cursor_position,
                update_in_game_ui_tooltip,
            )
                .chain()
                .run_if(in_state(GameState::InGame))
                .in_set(UiSystemSet::UpdateUi),
        );
    }
}
