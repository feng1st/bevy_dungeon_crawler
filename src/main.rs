#![warn(clippy::all, clippy::pedantic)]

use bevy::prelude::*;
use game_core::GameCorePlugin;
use game_ui::GameUiPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        GameCorePlugin,
        GameUiPlugin,
    ));
    #[cfg(debug_assertions)]
    {
        app.add_systems(Last, print_entity_count);
    }
    app.run();
}

#[cfg(debug_assertions)]
#[allow(clippy::needless_pass_by_value)]
fn print_entity_count(world: &World, mut timer: Local<u128>, time: Res<Time>) {
    *timer += time.delta().as_millis();
    if *timer > 1000 {
        *timer = 0;
        println!("Entity count: {}", world.entities().len());
    }
}
