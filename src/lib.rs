#![allow(unused_parens)]
use bevy::prelude::*;

use camera::CameraPlugin;
use game::GamePlugin;
use ui::UiPlugin;

mod camera;
mod game;
mod ui;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, GamePlugin, UiPlugin));
        app.add_state::<GameState>();
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    GameOver,
}
