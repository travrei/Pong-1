use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;

use crate::{paddles, walls};

const BALLSPEED: f32 = 250.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(
                FixedUpdate,
                (apply_velocity, check_for_collisions, score, display_score),
            )
            .insert_resource(ScoreP1 { score: 0 })
            .insert_resource(ScoreP2 { score: 0 });
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Resource, Deref, DerefMut)]
pub struct ScoreP1 {
    score: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ScoreP2 {
    score: usize,
}

fn spawn(mut commands: Commands) {
    let mut rng = thread_rng();
    let dir_x: f32 = rng.gen_range(-1.0..1.);
    let dir_y: f32 = rng.gen_range(-1.0..1.);

    let initial_direction = Vec2::new(dir_x, dir_y);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(250., 250., 250.),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 0.),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(initial_direction.normalize() * BALLSPEED),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query_walls: Query<(Entity, &Transform), With<walls::Collider>>,
    collider_query_paddles: Query<(Entity, &Transform), With<paddles::Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (_collider_entity, transform) in &collider_query_walls {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        let mut reflect_x = false;
        let mut reflect_y = false;

        match collision {
            Some(Collision::Left) => reflect_x = ball_velocity.x > 0.0,
            Some(Collision::Right) => reflect_x = ball_velocity.x < 0.0,
            Some(Collision::Top) => reflect_y = ball_velocity.y < 0.0,
            Some(Collision::Bottom) => reflect_y = ball_velocity.y > 0.0,
            Some(Collision::Inside) => { /* do nothing */ }
            None => {}
        }

        if reflect_x {
            ball_velocity.x = -ball_velocity.x;
        }

        if reflect_y {
            ball_velocity.y = -ball_velocity.y;
        }
    }
    for (_collider_entity, transform) in &collider_query_paddles {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        let mut reflect_x = false;
        let mut reflect_y = false;

        match collision {
            Some(Collision::Left) => reflect_x = ball_velocity.x > 0.0,
            Some(Collision::Right) => reflect_x = ball_velocity.x < 0.0,
            Some(Collision::Top) => reflect_y = ball_velocity.y < 0.0,
            Some(Collision::Bottom) => reflect_y = ball_velocity.y > 0.0,
            Some(Collision::Inside) => { /* do nothing */ }
            None => {}
        }

        if reflect_x {
            ball_velocity.x = -ball_velocity.x;
        }

        if reflect_y {
            ball_velocity.y = -ball_velocity.y;
        }
    }
}

fn score(
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &Ball)>,
    mut scorep1: ResMut<ScoreP1>,
    mut scorep2: ResMut<ScoreP2>,
) {
    let (mut ball_transform, _ball) = ball_query.single_mut();

    if ball_transform.translation.x > 1280. / 2. {
        scorep1.score += 1;
        ball_transform.translation = Vec3::new(0., 0., 0.);
    }
    if ball_transform.translation.x < -1280. / 2. {
        scorep2.score += 1;
        ball_transform.translation = Vec3::new(0., 0., 0.);
    }
}

fn display_score(scorep1: Res<ScoreP1>, scorep2: Res<ScoreP2>, mut commands: Commands) {
    commands.spawn(TextBundle::from_sections([TextSection::new(
        scorep1.score.to_string(),
        TextStyle {
            font_size: 60.,
            ..default()
        },
    )]));
    commands.spawn(
        TextBundle::from_sections([TextSection::new(
            scorep2.score.to_string(),
            TextStyle {
                font_size: 60.,
                ..default()
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
        }),
    );
    println!("P1: {:?} || P2: {:?}", scorep1.score, scorep2.score);
}
