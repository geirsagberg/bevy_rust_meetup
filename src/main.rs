use bevy::prelude::*;
use bevy_rust_meetup::GamePlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
