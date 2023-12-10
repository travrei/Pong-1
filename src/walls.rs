use bevy::prelude::*;

//Consts
const WALLSIZE: Vec2 = Vec2::new(1280., 20.);
const WALLCOLOR: Color = Color::rgb(250., 250., 250.);
const BOTTOMWALL: Vec3 = Vec3::new(0., -720. / 2., 0.);
const TOPWALL: Vec3 = Vec3::new(0., 720. / 2., 0.);
const DEVIDERPOS: Vec3 = Vec3::new(0., 0., -1.);
const DEVIDERCOLOR: Color = Color::rgba(250., 250., 250., 0.5);
const DEVIDERSIZE: Vec2 = Vec2::new(10., 720.);

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}
//Components
#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Devider;

#[derive(Component)]
pub struct Collider;

fn spawn(mut commands: Commands) {
    //Top Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALLCOLOR,
                custom_size: Some(WALLSIZE),
                ..default()
            },
            transform: Transform::from_translation(TOPWALL),
            ..default()
        },
        Wall,
        Collider,
    ));

    //Bottom Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALLCOLOR,
                custom_size: Some(WALLSIZE),
                ..default()
            },
            transform: Transform::from_translation(BOTTOMWALL),
            ..default()
        },
        Wall,
        Collider,
    ));

    //Devider
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: DEVIDERCOLOR,
                custom_size: Some(DEVIDERSIZE),
                ..default()
            },
            transform: Transform::from_translation(DEVIDERPOS),
            ..default()
        },
        Devider,
    ));
}
