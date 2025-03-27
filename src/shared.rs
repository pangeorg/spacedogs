/// Shared components for the game
use crate::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component, Default)]
pub struct Dead;

impl Health {
    pub fn dec(&mut self) {
        self.0 -= 1;
    }
}
