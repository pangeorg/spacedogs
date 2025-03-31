use std::f32::consts::PI;

use enemy::EnemyDiedEvent;
use enemy::EnemyPlugin;
use player::Player;
use player::PlayerPlugin;

use crate::prelude::constants::*;
use crate::prelude::physics::*;
use crate::prelude::*;

pub mod enemy;
pub mod equipment;
pub mod player;
pub mod shared;

#[derive(Component)]
pub struct Debris;

/// The plugin for everything in our world.
/// Here we add the player, enemies, other structures of the world, e.g. debris.
/// Also, dependent plugins, e.g., weapons, shields, ships should be added here.
/// UI and physics will be handled 'externally'.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyPlugin, PlayerPlugin)).add_systems(
            Update,
            (despawn_out_of_world, on_enemy_died_debris, despawn::<Dead>),
        );
    }
}

fn on_enemy_died_debris(
    mut commands: Commands,
    mut ev_enemy_died: EventReader<EnemyDiedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in ev_enemy_died.read() {
        for _ in 0..20 {
            let debris_mesh = meshes.add(Triangle2d::new(
                Vec2::Y * 2.0,
                Vec2::new(-2.0, -2.0),
                Vec2::new(2.0, -2.0),
            ));
            let color = Color::hsl(0.5, 0.35, 0.7);
            let r: f32 = rand::random();
            let x = (r * 2.0 * PI).cos();
            let y = (r * 2.0 * PI).sin();

            commands.spawn((
                Mesh2d(debris_mesh),
                MeshMaterial2d(materials.add(color)),
                Transform {
                    translation: event.position.extend(1.0),
                    ..default()
                },
                Debris,
                Velocity(Vec2::new(x, y) * PROJECTILE_SPEED),
            ));
        }
    }
}

fn despawn<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in &q {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_out_of_world(
    mut commands: Commands,
    object_query: Query<(Entity, &Transform), (Without<Player>, With<Collider>)>,
) {
    for (obj, transform) in &object_query {
        if transform.translation.y > TOP * 2.0 {
            commands.entity(obj).despawn();
        }
        if transform.translation.y < BOTTOM * 1.1 {
            commands.entity(obj).despawn();
        }
        if transform.translation.x < LEFT * 1.1 {
            commands.entity(obj).despawn();
        }
        if transform.translation.x > RIGHT * 1.1 {
            commands.entity(obj).despawn();
        }
    }
}
