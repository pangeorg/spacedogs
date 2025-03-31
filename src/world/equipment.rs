use crate::prelude::*;

pub enum ProjectileType {
    Single,
    Fan(f32),
    Burst,
}

#[derive(Component)]
pub struct Weapon {
    name: String,
    fire_rate: f32,
    damage: f32,
    projectile_speed: f32,
    projectile_type: ProjectileType,
    projectiles_per_shot: usize,
}
