#![warn(clippy::all, clippy::pedantic)]

use bevy::prelude::*;
use game_core::GameCorePlugin;
use game_ui::GameUiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            GameCorePlugin,
            GameUiPlugin,
        ))
        .run();
}
