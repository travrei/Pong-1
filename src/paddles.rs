use bevy::prelude::*;

const PADDLECOLOR: Color = Color::rgb(250., 250., 250.);
const PADDLESIZE: Vec3 = Vec3::new(20., 100., 0.);

pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, (move_paddle1, move_paddle2));
    }
}

#[derive(Component)]
struct Paddle1;

#[derive(Component)]
struct Paddle2;

#[derive(Component)]
pub struct Collider;

fn spawn(mut commands: Commands) {
    //PADDLE 1
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PADDLECOLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new((1280. / 2.) - 15., 0., 0.),
                scale: PADDLESIZE,
                ..default()
            },
            ..default()
        },
        Paddle1,
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PADDLECOLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new((-1280. / 2.) + 15., 0., 0.),
                scale: PADDLESIZE,
                ..default()
            },
            ..default()
        },
        Paddle2,
        Collider,
    ));
}

fn move_paddle1(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut paddle1_query: Query<(&mut Transform, &Paddle1)>,
) {
    for (mut transform, _paddle1) in &mut paddle1_query {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 150. * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 150. * time.delta_seconds();
        }
    }
}

fn move_paddle2(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut paddle2_query: Query<(&mut Transform, &Paddle2)>,
) {
    for (mut transform, _paddle2) in &mut paddle2_query {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 150. * time.delta_seconds();
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 150. * time.delta_seconds();
        }
    }
}
