use bevy::prelude::*;

use crate::{score::Score, GameState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), handle_game_over_event)
            .add_systems(OnExit(GameState::GameOver), cleanup)
            .add_systems(
                Update,
                click_to_restart.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_game_over_event(mut commands: Commands, query: Query<&Score>) {
    let score = query.single();

    let winner = if score.player > score.enemy {
        "Player"
    } else {
        "CPU"
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Game Over\n{winner} Won\nClick to restart"),
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
}

fn click_to_restart(
    mut state: ResMut<NextState<GameState>>,
    mouse_input: ResMut<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        state.0 = Some(GameState::Menu);
    }
}
