#![allow(unused_parens)]
use bevy::prelude::*;

mod assets;
mod background;
mod camera;
mod demo;
mod game;
mod game_over;
mod inspector;
mod materials;
mod music;
mod score;
mod ui;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            demo::DemoPlugin,
            // game::GamePlugin,
            // ui::UiPlugin,
            // score::ScorePlugin,
            // game_over::GameOverPlugin,
            // assets::LoadingPlugin,
            // background::BackgroundPlugin,
            // music::MusicPlugin,
            // inspector::InspectorPlugin,
        ))
        .add_state::<GameState>()
        .add_event::<ResetEvent>()
        .add_event::<ScoreEvent>();
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
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
