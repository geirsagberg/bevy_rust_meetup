use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stuff)
            .add_systems(Update, move_stuff);
    }
}

fn spawn_stuff(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::WHITE,
            ..default()
        },
        ..default()
    });
}

fn move_stuff(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 10.0;
        }
    }
}
