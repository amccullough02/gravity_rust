use bevy::prelude::*;

mod fps_counter;
use fps_counter::{fps_counter_system, setup_fps_counter};

mod body;
use body::{Body, spawn_body};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 800.0).into(),
                title: "N-body Sim".to_string(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_fps_counter, setup_simulation, setup_camera))
        .add_systems(Update, (fps_counter_system, update_bodies))
        .run();
}

fn setup_simulation(mut commands: Commands) {
    spawn_body(&mut commands, Body::new(0.0, 0.0, 50.0, 1.989e30, Color::WHITE));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_bodies(mut query: Query<(&mut Transform, &Body)>) {
    for (mut transform, body) in query.iter_mut() {
        transform.translation = body.position.extend(0.0);
    }
}
