use bevy::math::NormedVectorSpace;

use crate::prelude::*;
use crate::{constants::*, physics::*};

#[derive(Event)]
pub struct EnemyDiedEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub enemy_type: EnemyType,
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
            .add_systems(Update, (spawn_enemies, on_enemy_died, follow_path));
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
