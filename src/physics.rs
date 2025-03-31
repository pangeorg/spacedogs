use crate::prelude::*;

use bevy::{
    ecs::entity,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
};

#[derive(Component)]
pub struct Momentum(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}

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
    mut q: Query<(Entity, &Transform, &Name), With<Collider>>,
    mut colission_events: EventWriter<CollisionEvent>,
) {
    let mut combos = q.iter_combinations_mut();
    while let Some([(e1, trans1, name1), (e2, trans2, name2)]) = combos.fetch_next() {
        if e1 == e2 {
            continue;
        }

        let collision = collide(
            Aabb2d::new(trans1.translation.truncate(), trans1.scale.truncate()),
            Aabb2d::new(trans2.translation.truncate(), trans2.scale.truncate()),
        );

        if collision {
            colission_events.send(CollisionEvent {
                entity1: e1,
                entity2: e2,
            });
        }
    }
}

//fn check_for_collisions(
//    mut commands: Commands,
//    mut score: ResMut<Score>,
//    player_query: Single<&Transform, With<Player>>,
//    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
//    mut enemy_query: Query<(Entity, &Transform, &mut Health, &Enemy), With<Enemy>>,
//    mut enemy_died_events: EventWriter<EnemyDiedEvent>,
//) {
//    let player_transform = player_query.into_inner();
//
//    // check if player is hit by enemy
//    for (enemy_id, enemy_transform, mut enemy_health, enemy) in &mut enemy_query {
//        let player_collision = collision(
//            BoundingCircle::new(enemy_transform.translation.truncate(), 1.0 / 2.),
//            Aabb2d::new(
//                player_transform.translation.truncate(),
//                player_transform.scale.truncate() / 2.,
//            ),
//        );
//
//        if player_collision {
//            // Sends a collision event so that other systems can react to the collision
//            commands.entity(enemy_id).despawn();
//            **score -= 1;
//        }
//
//        for (projectile, projectile_transform) in &projectile_query {
//            let projectile_collision = collision(
//                BoundingCircle::new(enemy_transform.translation.truncate(), 30.),
//                Aabb2d::new(
//                    projectile_transform.translation.truncate(),
//                    projectile_transform.scale.truncate() / 2.,
//                ),
//            );
//
//            if projectile_collision {
//                enemy_health.dec();
//                if enemy_health.0 <= 0 {
//                    enemy_died_events.send(EnemyDiedEvent {
//                        entity: enemy_id,
//                        enemy_type: enemy.enemy_type.to_owned(),
//                        position: projectile_transform.translation.truncate(),
//                    });
//                }
//                commands.entity(projectile).despawn();
//            }
//        }
//    }
//}

#[inline]
fn collide(a: Aabb2d, b: Aabb2d) -> bool {
    return a.intersects(&b);
}
