use bevy::prelude::App;
use spacedogs::GamePlugin;

fn main() {
    App::new().add_plugins((GamePlugin,)).run();
}
