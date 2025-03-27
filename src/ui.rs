use crate::prelude::constants::*;
use crate::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct ScoreboardUi;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, (init_scoreboard,))
            .add_systems(Update, (update_scoreboard, on_enemy_died_score));
    }
}

fn init_scoreboard(mut commands: Commands) {
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

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn on_enemy_died_score(mut score: ResMut<Score>, mut ev_enemy_died: EventReader<EnemyDiedEvent>) {
    for event in ev_enemy_died.read() {
        match event.enemy_type {
            EnemyType::Creep => **score += 1,
            EnemyType::Standard => **score += 2,
        }
    }
}
