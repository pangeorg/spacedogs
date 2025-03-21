use crate::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

impl Health {
    pub fn dec(&mut self) {
        self.0 -= 1;
    }
}
