use bevy::{prelude::*, window::WindowResolution};

mod ball;
mod paddles;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "20-Games - 1.Pong".to_string(),
                resizable: false,
                resolution: WindowResolution::new(1280., 720.),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(walls::WallPlugin)
        .add_plugins(paddles::PaddlesPlugin)
        .add_plugins(ball::BallPlugin)
        .add_systems(Startup, camera_spawn)
        .run();
}

fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
