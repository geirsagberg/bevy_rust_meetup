use bevy::prelude::*;

use crate::{game::ARENA_HEIGHT, GameState, ScoreEvent};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_score)
            .add_systems(
                Update,
                handle_score_event.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_score(mut commands: Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0 0",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, ARENA_HEIGHT / 2. - 40., 0.0)),
            ..default()
        },
        Score::default(),
    ));
}

const MAX_SCORE: u32 = 3;

#[derive(Component, Default)]
pub struct Score {
    pub player: u32,
    pub enemy: u32,
}

fn handle_score_event(
    mut score_query: Query<(&mut Score, &mut Text)>,
    mut score_events: EventReader<ScoreEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for event in score_events.read() {
        println!("Score event: {:?}", event);
        let (mut score, mut text) = score_query.single_mut();
        if event.player {
            score.player += 1;
        } else {
            score.enemy += 1;
        }
        text.sections[0].value = format!("{} {}", score.player, score.enemy);
        if score.player == MAX_SCORE || score.enemy == MAX_SCORE {
            state.0 = Some(GameState::GameOver);
        }
    }
}
