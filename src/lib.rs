#![allow(unused_parens)]
use bevy::prelude::*;

use camera::CameraPlugin;
use game::GamePlugin;
use game_over::GameOverPlugin;
use score::ScorePlugin;
use ui::UiPlugin;

mod camera;
mod game;
mod game_over;
mod score;
mod ui;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            GamePlugin,
            UiPlugin,
            ScorePlugin,
            GameOverPlugin,
        ))
        .add_state::<GameState>()
        .add_event::<ResetEvent>()
        .add_event::<ScoreEvent>();
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    GameOver,
}

#[derive(Event, Debug)]
pub struct ScoreEvent {
    pub player: bool,
}

#[derive(Event, Debug)]
pub struct ResetEvent;
