use bevy::{
    prelude::*,
    render::settings::{Backends, WgpuSettings},
    window::close_on_esc,
};

mod ball;
mod paddles;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                title: "1 - Pong!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((walls::WallsPlugin, paddles::PaddlesPlugin))
        .add_plugins(ball::BallPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, close_on_esc)
        .run();
}

// Camera System
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
