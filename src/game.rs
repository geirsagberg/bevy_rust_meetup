use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{GameState, ResetEvent, ScoreEvent};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_paddles, spawn_ball, spawn_walls))
            .add_systems(Update, handle_reset_event)
            .add_systems(
                Update,
                (
                    (move_player_paddle, move_cpu_paddle, move_ball),
                    clamp_paddles,
                    (check_collisions, check_goals),
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

pub const ARENA_WIDTH: f32 = 800.0;
pub const ARENA_HEIGHT: f32 = 600.0;

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Cpu;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Goal {
    pub player: bool,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Resettable {
    pub initial_position: Vec2,
}

const PADDLE_SPEED: f32 = 5.0;

fn handle_reset_event(
    mut reset_events: EventReader<ResetEvent>,
    mut query: Query<(&mut Transform, &Resettable)>,
) {
    if let Some(_) = reset_events.read().next() {
        for (mut transform, resettable) in &mut query {
            transform.translation = Vec3::new(
                resettable.initial_position.x,
                resettable.initial_position.y,
                0.0,
            );
        }
    }
}

fn move_player_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform), (With<Player>, With<Paddle>)>,
) {
    for (mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += PADDLE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= PADDLE_SPEED;
        }
    }
}

fn move_cpu_paddle(
    mut query: Query<(&mut Transform), (With<Cpu>, With<Paddle>)>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
) {
    for (mut transform) in &mut query {
        for ball_transform in &ball_query {
            if ball_transform.translation.y > transform.translation.y {
                transform.translation.y += PADDLE_SPEED;
            }
            if ball_transform.translation.y < transform.translation.y {
                transform.translation.y -= PADDLE_SPEED;
            }
        }
    }
}

fn clamp_paddles(mut query: Query<(&mut Transform, &Sprite), With<Paddle>>) {
    for (mut transform, sprite) in &mut query {
        if transform.translation.y > ARENA_HEIGHT / 2.0 - sprite.custom_size.unwrap().y / 2.0 {
            transform.translation.y = ARENA_HEIGHT / 2.0 - sprite.custom_size.unwrap().y / 2.0;
        }
        if transform.translation.y < -ARENA_HEIGHT / 2.0 + sprite.custom_size.unwrap().y / 2.0 {
            transform.translation.y = -ARENA_HEIGHT / 2.0 + sprite.custom_size.unwrap().y / 2.0;
        }
    }
}

fn move_ball(mut query: Query<(&mut Transform, &Velocity), With<Ball>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

fn check_goals(
    ball_query: Query<(&Transform, &Sprite), With<Ball>>,
    goal_query: Query<(&Transform, &Sprite, &Goal)>,
    mut score_events: EventWriter<ScoreEvent>,
    mut reset_events: EventWriter<ResetEvent>,
) {
    for (ball_transform, ball_sprite) in &ball_query {
        for (goal_transform, goal_sprite, goal) in &goal_query {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.custom_size.unwrap(),
                goal_transform.translation,
                goal_sprite.custom_size.unwrap(),
            );

            if collision.is_some() {
                score_events.send(ScoreEvent {
                    player: goal.player,
                });
                reset_events.send(ResetEvent);
            }
        }
    }
}

fn check_collisions(
    mut ball_query: Query<(&Transform, &Sprite, &mut Velocity), With<Ball>>,
    paddle_query: Query<(&Transform, &Sprite), With<Paddle>>,
    wall_query: Query<(&Transform, &Sprite), With<Wall>>,
) {
    for (ball_transform, ball_sprite, mut ball_velocity) in &mut ball_query {
        for (paddle_transform, paddle_sprite) in &paddle_query {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.custom_size.unwrap(),
                paddle_transform.translation,
                paddle_sprite.custom_size.unwrap(),
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.0;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.0;
                    }
                    _ => {}
                }
            }
        }

        for (wall_transform, wall_sprite) in &wall_query {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.custom_size.unwrap(),
                wall_transform.translation,
                wall_sprite.custom_size.unwrap(),
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.0;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.0;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn spawn_paddles(mut commands: Commands) {
    let player_position = Vec2::new(-ARENA_WIDTH / 2.0 + 20.0, 0.0);
    let cpu_position = Vec2::new(ARENA_WIDTH / 2.0 - 20.0, 0.0);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(player_position.x, player_position.y, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Player,
        Resettable {
            initial_position: player_position,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(cpu_position.x, cpu_position.y, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Cpu,
        Resettable {
            initial_position: cpu_position,
        },
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
        Velocity(Vec2::new(5.0, 5.0)),
        Resettable {
            initial_position: Vec2::new(0.0, 0.0),
        },
    ));
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, ARENA_HEIGHT / 2., 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(ARENA_WIDTH, 10.0)),
                ..default()
            },
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -ARENA_HEIGHT / 2., 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(ARENA_WIDTH, 10.0)),
                ..default()
            },
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-ARENA_WIDTH / 2., 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, ARENA_HEIGHT + 10.0)),
                ..default()
            },
            ..default()
        },
        Goal { player: false },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(ARENA_WIDTH / 2., 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, ARENA_HEIGHT + 10.0)),
                ..default()
            },
            ..default()
        },
        Goal { player: true },
    ));
}
