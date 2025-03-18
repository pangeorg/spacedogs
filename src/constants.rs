use bevy::prelude::*;
pub const VIEWPORT_WIDTH: f32 = 1920.;
pub const VIEWPORT_HEIGHT: f32 = 1080.;
// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 300.0;
// How close can the paddle get to the wall
pub const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
pub const PROJECTILE_SPEED: f32 = 450.0;
pub const INITITAL_PROJECTILE_DIRECTION: Vec2 = Vec2::new(0., 1.);
pub const PROJECTILE_SIZE: f32 = 30.;
pub const PROJECTILE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

// x coordinates
pub const RIGHT: f32 = VIEWPORT_WIDTH / 2.;
pub const LEFT: f32 = -RIGHT;
// y coordinates
pub const TOP: f32 = VIEWPORT_HEIGHT / 2.;
pub const BOTTOM: f32 = -TOP;

pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
