use crate::prelude::constants::*;
use crate::prelude::physics::*;
use crate::prelude::ui::*;
use crate::prelude::*;

use std::f32::consts::PI;

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

#[derive(Component)]
pub struct Debris;

// everything which 'lives' but is not the player
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDiedEvent>()
            .add_systems(Update, (despawn_out_of_world, spawn_enemies, on_enemy_died));
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
        if transform.translation.y < BOTTOM * 2.0 {
            commands.entity(obj).despawn();
        }
        if transform.translation.x < LEFT * 2.0 {
            commands.entity(obj).despawn();
        }
        if transform.translation.x > RIGHT * 2.0 {
            commands.entity(obj).despawn();
        }
    }
}

fn on_enemy_died(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut ev_enemy_died: EventReader<EnemyDiedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in ev_enemy_died.read() {
        commands.entity(event.entity).despawn_recursive();

        match event.enemy_type {
            EnemyType::Creep => **score += 1,
            EnemyType::Standard => **score += 2,
        }

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
