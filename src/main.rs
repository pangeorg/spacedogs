use std::{f32::consts::PI, ops::Deref};

use bevy::{
    ecs::event,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
    sprite::Anchor,
    transform,
};

mod constants;
use constants::*;

fn main() {
    // Define window
    let primary_window = Window {
        title: "Spacedogs".to_string(),
        resolution: (VIEWPORT_WIDTH / 2., VIEWPORT_HEIGHT / 2.).into(),
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Score(0))
        .add_event::<CollisionEvent>()
        .add_event::<EnemyDiedEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 65 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_player,
                player_shoot,
                spawn_enemies,
                check_for_collisions,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(
            Update,
            (
                update_scoreboard,
                despawn_out_of_world,
                move_background,
                on_enemy_died,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    fire_rate: f32,
    last_shot: f32,
}

impl Player {
    pub fn new(fire_rate: f32) -> Self {
        Player {
            fire_rate,
            last_shot: 0.,
        }
    }

    pub fn set_last_shot(&mut self, last_shot: f32) {
        self.last_shot = last_shot;
    }
}

#[derive(Component)]
struct Momentum(Vec2);

#[derive(Component)]
struct PlayerProjectile;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Event)]
struct EnemyDiedEvent {
    entity: Entity,
    position: Vec2,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Debris;

impl Health {
    pub fn dec(&mut self) {
        self.0 -= 1;
    }
}

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
struct Score(i32);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct Background;

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: VIEWPORT_WIDTH,
                height: VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Background
    commands.spawn((
        Background,
        Sprite {
            image: asset_server.load("backgrounds/Starfields/Starfield_01-1024x1024.png"),
            custom_size: Some(Vec2::new(VIEWPORT_WIDTH, VIEWPORT_WIDTH)),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, BOTTOM - 10.0, -1.0),
            ..default()
        },
    ));
    commands.spawn((
        Background,
        Sprite {
            image: asset_server.load("backgrounds/Starfields/Starfield_01-1024x1024.png"),
            custom_size: Some(Vec2::new(VIEWPORT_WIDTH, VIEWPORT_WIDTH)),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, TOP, -1.0),
            ..default()
        },
    ));

    // Player
    let player_y = BOTTOM + GAP_BETWEEN_PADDLE_AND_FLOOR;

    let player_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 30.0,
        Vec2::new(-30.0, -30.0),
        Vec2::new(30.0, -30.0),
    ));
    let color = Color::hsl(0.8, 0.95, 0.7);

    commands.spawn((
        Mesh2d(player_mesh),
        MeshMaterial2d(materials.add(color)),
        Transform {
            translation: Vec3::new(0.0, player_y, 0.0),
            ..default()
        },
        Player::new(0.2),
        Collider,
        Momentum(Vec2::new(0., 0.)),
    ));

    // Scoreboard
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

fn move_background(mut bg_query: Query<&mut Transform, With<Background>>) {
    for mut transform in &mut bg_query {
        transform.translation.y -= 5.0;
        if transform.translation.y < BOTTOM - VIEWPORT_WIDTH {
            transform.translation.y = TOP;
        }
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut Transform, &mut Momentum), With<Player>>,
    time: Res<Time>,
) {
    let (mut paddle_transform, mut momentum) = query.into_inner();

    let mut dx = 0.0;
    let mut dy = 0.0;
    let mut mx = momentum.0.x;
    let mut my = momentum.0.y;
    let mut pressed = false;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        dx -= 1.0;
        mx -= 0.1;
        pressed = true;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        dx += 1.0;
        mx += 0.1;
        pressed = true;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        dy -= 1.0;
        my -= 0.1;
        pressed = true;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        dy += 1.0;
        my += 0.1;
        pressed = true;
    }

    if !pressed {
        mx *= 0.91;
        my *= 0.91;
    }

    mx = mx.clamp(-1., 1.);
    my = my.clamp(-1., 1.);

    momentum.0.x = mx;
    momentum.0.y = my;

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_xposition =
        paddle_transform.translation.x + ((dx + mx) * PADDLE_SPEED) * time.delta_secs();
    let new_paddle_yposition =
        paddle_transform.translation.y + ((dy + my) * PADDLE_SPEED) * time.delta_secs();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    let upper_bound = TOP - PADDLE_SIZE.y / 2.0;
    let lower_bound = BOTTOM + PADDLE_SIZE.y / 2.0;

    paddle_transform.translation.x = new_paddle_xposition.clamp(left_bound, right_bound);
    paddle_transform.translation.y = new_paddle_yposition.clamp(lower_bound, upper_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Single<(&mut Player, &Transform), With<Player>>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let (mut player, player_transform) = player_query.into_inner();

    let t = time.elapsed_secs();

    if player.last_shot > 0. {
        if player.last_shot + player.fire_rate > t {
            return;
        }
    }

    player.set_last_shot(t);

    let pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    let projectile_mesh = meshes.add(Ellipse::new(5.0, 10.0));
    commands.spawn((
        Mesh2d(projectile_mesh),
        MeshMaterial2d(materials.add(PROJECTILE_COLOR)),
        Transform {
            translation: pos.extend(0.0),
            ..default()
        },
        PlayerProjectile,
        Collider,
        Velocity(INITITAL_PROJECTILE_DIRECTION.normalize() * PROJECTILE_SPEED),
        PointLight::default(),
    ));
}

fn despawn_out_of_world(
    mut commands: Commands,
    object_query: Query<(Entity, &Transform), (Without<Player>, With<Collider>)>,
) {
    for (obj, transform) in &object_query {
        if transform.translation.y > TOP {
            commands.entity(obj).despawn();
        }
        if transform.translation.y < BOTTOM {
            commands.entity(obj).despawn();
        }
        if transform.translation.x < LEFT {
            commands.entity(obj).despawn();
        }
        if transform.translation.x > RIGHT {
            commands.entity(obj).despawn();
        }
    }
}

fn spawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    let n_enemies = enemy_query.iter().count();
    if n_enemies > 20 {
        return;
    }

    let x = rand::random_range(LEFT..RIGHT);
    let y = TOP;
    let pos = Vec2::new(x, y);

    commands.spawn((
        Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        Transform {
            translation: pos.extend(0.0),
            scale: Vec3::new(PROJECTILE_SIZE, PROJECTILE_SIZE, 1.0),
            ..default()
        },
        Enemy,
        Health(2),
        Collider,
        Velocity(Vec2::new(0., -1.) * PROJECTILE_SPEED),
    ));
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

fn on_enemy_died(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut ev_enemy_died: EventReader<EnemyDiedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in ev_enemy_died.read() {
        commands.entity(event.entity).despawn();
        **score += 1;

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

// Returns `True` if `projectile` collides with `bounding_box`.
fn collision(projectile: BoundingCircle, bounding_box: Aabb2d) -> bool {
    return projectile.intersects(&bounding_box);
}
