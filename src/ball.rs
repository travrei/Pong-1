use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;

//Const
const BALLSIZE: Vec2 = Vec2::new(10., 10.);
const BALLCOLOR: Color = Color::rgb(250., 250., 250.);
const BALLSPEED: f32 = 300.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, (apply_velocity, check_for_collisions).chain());
    }
}

//Components
#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Collider;

fn spawn(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let initialdir = Vec2::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BALLCOLOR,
                custom_size: Some(BALLSIZE),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(initialdir.normalize() * BALLSPEED),
        Collider,
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Ball>)>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    //Check Collisions
    for (collider_entity, transform) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            BALLSIZE,
            transform.translation,
            transform.scale.truncate(),
        );

        let mut reflect_x = false;
        let mut reflect_y = false;

        match collision {
            Some(Collision::Left) => reflect_x = ball_velocity.0.x > 0.0,
            Some(Collision::Right) => reflect_x = ball_velocity.0.x < 0.0,
            Some(Collision::Top) => reflect_y = ball_velocity.0.y < 0.0,
            Some(Collision::Bottom) => reflect_y = ball_velocity.0.y > 0.0,
            Some(Collision::Inside) => { /* Faz nada carai */ }
            None => {}
        }

        if reflect_x {
            ball_velocity.0.x = -ball_velocity.0.x
        }
        if reflect_y {
            ball_velocity.0.y = -ball_velocity.0.y
        }
    }
}
