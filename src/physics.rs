use crate::prelude::player::*;
use crate::prelude::ui::*;
use crate::prelude::world::*;
use crate::prelude::*;

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};

#[derive(Component)]
pub struct Momentum(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (apply_velocity, check_for_collisions))
            .add_event::<CollisionEvent>();
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Single<&Transform, With<Player>>,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    mut enemy_died_events: EventWriter<EnemyDiedEvent>,
) {
    let player_transform = player_query.into_inner();

    // check if player is hit by enemy
    for (enemy, enemy_transform, mut enemy_health) in &mut enemy_query {
        let player_collision = collision(
            BoundingCircle::new(enemy_transform.translation.truncate(), 1.0 / 2.),
            Aabb2d::new(
                player_transform.translation.truncate(),
                player_transform.scale.truncate() / 2.,
            ),
        );

        if player_collision {
            // Sends a collision event so that other systems can react to the collision
            commands.entity(enemy).despawn();
            **score -= 1;
        }

        for (projectile, projectile_transform) in &projectile_query {
            let projectile_collision = collision(
                BoundingCircle::new(enemy_transform.translation.truncate(), 30.),
                Aabb2d::new(
                    projectile_transform.translation.truncate(),
                    projectile_transform.scale.truncate() / 2.,
                ),
            );

            if projectile_collision {
                enemy_health.dec();
                if enemy_health.0 <= 0 {
                    enemy_died_events.send(EnemyDiedEvent {
                        entity: enemy,
                        position: projectile_transform.translation.truncate(),
                    });
                }
                commands.entity(projectile).despawn();
            }
        }
    }
}

// Returns `True` if `projectile` collides with `bounding_box`.
fn collision(projectile: BoundingCircle, bounding_box: Aabb2d) -> bool {
    return projectile.intersects(&bounding_box);
}
