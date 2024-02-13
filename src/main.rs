use bevy::prelude::*;
use bevy_rust_meetup::MainPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".to_string(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            }),
            MainPlugin,
        ))
        .run();
}
