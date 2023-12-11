use bevy::prelude::*;

const WALLSIZE: Vec2 = Vec2::new(1280., 20.);
const WALLCOLOR: Color = Color::rgb(250., 250., 250.);

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

#[derive(Component)]
pub struct Collider;

fn spawn(mut commands: Commands) {
    //TOP WALL
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALLCOLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: (720. / 2.) - 10.,
                    z: 0.,
                },
                scale: WALLSIZE.extend(1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    // BOTTOM WALL
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALLCOLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: (-720. / 2.) + 10.,
                    z: 0.,
                },
                scale: WALLSIZE.extend(1.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    //Devider
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(250., 250., 250., 0.5),
            custom_size: Some(Vec2::new(10., 720.)),
            ..default()
        },
        ..default()
    });
}
