use bevy::prelude::*;
use bevy_rust_meetup::MainPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}
