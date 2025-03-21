mod constants;
mod game;
mod graphics;
mod physics;
mod player;
mod prelude;
mod shared;
mod ui;
mod world;

use game::GamePlugin;
use prelude::constants::*;
use prelude::*;

fn main() {
    let primary_window = Window {
        title: "Spacedogs".to_string(),
        resolution: (VIEWPORT_WIDTH / 2., VIEWPORT_HEIGHT / 2.).into(),
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run();
}
