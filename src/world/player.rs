use crate::prelude::constants::*;
use crate::prelude::physics::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    fire_rate: f32,
    last_shot: f32,
}

#[derive(Event, Default)]
pub struct PlayerHitByEnemyEvent;

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
pub struct PlayerProjectile;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitByEnemyEvent>()
            .add_systems(Startup, (spawn,))
            .add_systems(Update, (control, shoot, on_collision));
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Player
    let player_y = BOTTOM + GAP_BETWEEN_PADDLE_AND_FLOOR;

    let player_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 30.0,
        Vec2::new(-30.0, -30.0),
        Vec2::new(30.0, -30.0),
    ));
    let color = Color::hsl(0.8, 0.95, 0.7);

    commands.spawn((
        Name::new("Player"),
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
}

fn on_collision(
    mut colission_events: EventReader<CollisionEvent>,
    mut player_enemy_colission_events: EventWriter<PlayerHitByEnemyEvent>,
    player_query: Single<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let player_entity = player_query.into_inner();
    for event in colission_events.read() {
        let (e1, e2) = (event.entity1, event.entity2);
        let is_player = e1 == player_entity || e2 == player_entity;

        if is_player && (enemy_query.contains(e1) || enemy_query.contains(e2)) {
            let player_hit_by_enemy_event = PlayerHitByEnemyEvent::default();
            player_enemy_colission_events.send(player_hit_by_enemy_event);
        }
    }
}

fn control(
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

// TODO: maybe we put all keyboard interactions into one and then send events
fn shoot(
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
        Name::new("PlayerProjectile"),
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
