use bevy::prelude::*;

const PADDLESIZE: Vec2 = Vec2::new(10., 120.);
const PADDLECOLLOR: Color = Color::rgb(250., 250., 250.);
const PADDLESPAWN2: Vec3 = Vec3::new((1280. / 2.) - 15., 0., 0.);
const PADDLESPAWN1: Vec3 = Vec3::new((-1280. / 2.) + 15., 0., 0.);

pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (player1_moviment, player2_moviment));
    }
}
#[derive(Component)]
pub struct Paddle1;

#[derive(Component)]
pub struct Paddle2;

#[derive(Component)]
pub struct Collider;

fn spawn(mut commands: Commands) {
    //Player 1
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PADDLECOLLOR,
                custom_size: Some(PADDLESIZE),
                ..default()
            },
            transform: Transform::from_translation(PADDLESPAWN1),
            ..default()
        },
        Paddle1,
        Collider,
    ));

    //Player 2
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PADDLECOLLOR,
                custom_size: Some(PADDLESIZE),
                ..default()
            },
            transform: Transform::from_translation(PADDLESPAWN2),
            ..default()
        },
        Paddle2,
        Collider,
    ));
}

fn player1_moviment(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Paddle1)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut transform, _player1) in &mut player_query {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 150. * dt;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 150. * dt;
        }
    }
}

fn player2_moviment(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Paddle2)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut transform, _player2) in &mut player_query {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 150. * dt;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 150. * dt;
        }
    }
}
