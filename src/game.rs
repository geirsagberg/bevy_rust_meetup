use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_paddles, spawn_ball));
    }
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Cpu;

#[derive(Component)]
pub struct Ball;

fn spawn_paddles(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-380.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 100.0)),
                ..default()
            },
            ..default()
        },
        Paddle,
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(380.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 100.0)),
                ..default()
            },
            ..default()
        },
        Paddle,
        Cpu,
    ));
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        Ball,
    ));
}
