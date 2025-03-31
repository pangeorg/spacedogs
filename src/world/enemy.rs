use bevy::math::NormedVectorSpace;

use crate::helpers::poly_path::PolyPath;
use crate::prelude::*;
use crate::{constants::*, physics::*};

use super::player::PlayerProjectile;

#[derive(Event)]
pub struct EnemyDiedEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub enemy_type: EnemyType,
}

#[derive(Event)]
pub struct EnemyHitEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub damage: i32,
}

#[derive(Clone)]
pub enum EnemyType {
    Creep,
    Standard,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        Self { enemy_type }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDiedEvent>()
            .add_event::<EnemyHitEvent>()
            .add_systems(Update, (on_collision, on_hit, on_enemy_died).chain())
            .add_systems(Update, (spawn_enemies, follow_path));
    }
}

fn on_collision(
    mut commands: Commands,
    mut colission_events: EventReader<CollisionEvent>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
    projectile_query: Query<Entity, With<PlayerProjectile>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    fn send_event(
        enemy: Entity,
        projectile: Entity,
        q: &Query<(Entity, &Transform), With<Enemy>>,
        writer: &mut EventWriter<EnemyHitEvent>,
        commands: &mut Commands,
    ) {
        let (enemy, transform) = q.get(enemy).unwrap();
        writer.send(EnemyHitEvent {
            damage: 1,
            position: transform.translation.truncate(),
            entity: enemy,
        });
        // TODO: Make this an extra event + cleanup?
        commands.entity(projectile).despawn_recursive();
    }

    for event in colission_events.read() {
        let (e1, e2) = (event.entity1, event.entity2);

        if projectile_query.contains(e1) && enemy_query.contains(e2) {
            send_event(e2, e1, &enemy_query, &mut enemy_hit_events, &mut commands);
        }

        if projectile_query.contains(e2) && enemy_query.contains(e1) {
            send_event(e1, e2, &enemy_query, &mut enemy_hit_events, &mut commands);
        }
    }
}

fn on_hit(
    mut events: EventReader<EnemyHitEvent>,
    mut q: Query<(Entity, &mut Health), With<Enemy>>,
    mut died: EventWriter<EnemyDiedEvent>,
) {
    for event in events.read() {
        if q.contains(event.entity) {
            let e = event.entity;
            if let Ok((_, mut health)) = q.get_mut(e) {
                if health.0 == 1 {
                    died.send(EnemyDiedEvent {
                        entity: e,
                        position: event.position,
                        enemy_type: EnemyType::Creep,
                    });
                } else {
                    health.0 -= 1;
                }
            }
        }
    }
}

fn follow_path(mut query: Query<(&mut Velocity, &Transform, &mut PolyPath)>, time: Res<Time>) {
    for (mut velocity, transform, mut path) in &mut query {
        let vabs = velocity.norm();
        let dx = vabs * time.delta_secs();
        let pos = transform.translation.truncate();
        let next_pos = path.step(dx);
        let dir = (next_pos - pos).normalize();
        velocity.0 = vabs * dir;
    }
}

fn on_enemy_died(mut commands: Commands, mut ev_enemy_died: EventReader<EnemyDiedEvent>) {
    for event in ev_enemy_died.read() {
        commands.entity(event.entity).insert(Dead);
    }
}

fn spawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    let n_enemies = enemy_query.iter().count();
    if n_enemies > 0 {
        return;
    }

    //let x = rand::random_range(LEFT..RIGHT);
    let x = 0.;
    let y = TOP - 100.0;
    let pos = Vec2::new(x, y);

    //commands.spawn((
    //    Sprite {
    //        color: BRICK_COLOR,
    //        ..default()
    //    },
    //    Transform {
    //        translation: pos.extend(0.0),
    //        scale: Vec3::new(PROJECTILE_SIZE, PROJECTILE_SIZE, 1.0),
    //        ..default()
    //    },
    //    Enemy::new(EnemyType::Creep),
    //    Health(2),
    //    Collider,
    //    Velocity(Vec2::new(0., -1.) * PROJECTILE_SPEED),
    //));

    if rand::random_range(0.0..1.0) > 0.95 {
        // once in a while spawn a triangle path enemy

        let pos = Vec2::new(LEFT / 7.0, y);
        let path = PolyPath::new(vec![pos, Vec2::new(RIGHT / 7.0, y), Vec2::new(0., 0.)]);

        commands.spawn((
            Name::new("Enemy"),
            Sprite {
                color: BRICK_COLOR,
                ..default()
            },
            Transform {
                translation: pos.extend(0.0),
                scale: Vec3::new(PROJECTILE_SIZE * 2.1, PROJECTILE_SIZE * 2.1, 1.0),
                ..default()
            },
            Enemy::new(EnemyType::Standard),
            Health(4),
            Collider,
            Velocity(Vec2::new(0., -1.) * 300.),
            path,
        ));
    }

    //if rand::random_range(0.0..1.0) > 0.9 {
    //    // once in a while spawn a larger enemy
    //    commands.spawn((
    //        Sprite {
    //            color: BRICK_COLOR,
    //            ..default()
    //        },
    //        Transform {
    //            translation: pos.extend(0.0),
    //            scale: Vec3::new(PROJECTILE_SIZE * 2.1, PROJECTILE_SIZE * 2.1, 1.0),
    //            ..default()
    //        },
    //        Enemy::new(EnemyType::Standard),
    //        Health(4),
    //        Collider,
    //        Velocity(Vec2::new(0., -1.) * PROJECTILE_SPEED),
    //    ));
    //}
}
