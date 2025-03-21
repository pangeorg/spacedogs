use crate::{
    graphics::GraphicsPlugin, physics::PhysicsPlugin, player::PlayerPlugin, prelude::*,
    ui::UiPlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GraphicsPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            WorldPlugin,
            UiPlugin,
        ));
    }
}
