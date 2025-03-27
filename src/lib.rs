mod constants;
mod enemy;
mod graphics;
mod helpers;
mod physics;
mod player;
mod prelude;
mod shared;
mod ui;
mod world;

use constants::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

use crate::{
    graphics::GraphicsPlugin, physics::PhysicsPlugin, player::PlayerPlugin, prelude::*,
    ui::UiPlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let primary_window = Window {
            title: "Spacedogs".to_string(),
            resolution: (VIEWPORT_WIDTH / 2., VIEWPORT_HEIGHT / 2.).into(),
            ..default()
        };
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GraphicsPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            WorldPlugin,
            UiPlugin,
        ));
    }
}
