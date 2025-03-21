use crate::prelude::constants::*;
use crate::prelude::*;
use bevy::{render::camera::ScalingMode, sprite::Anchor};

#[derive(Component)]
struct Background;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,))
            .add_systems(Update, (move_background,));
    }
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn move_background(mut bg_query: Query<&mut Transform, With<Background>>) {
    for mut transform in &mut bg_query {
        transform.translation.y -= 5.0;
        if transform.translation.y < BOTTOM - VIEWPORT_WIDTH {
            transform.translation.y = TOP;
        }
    }
}
