use bevy::prelude::*;

mod fps_counter;
use fps_counter::{fps_counter_system, setup_fps_counter};

mod body;
use body::{Body, spawn_body};

mod constants;
use constants::{AU, G, SCALE, TIMESTEP};

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

    let mut bodies = Vec::new();
    
    // Sun
    let sun = spawn_body(&mut commands, Body::new(0.0, 0.0, 30.0, 1.989e30, Color::WHITE, true));
    bodies.push(sun);

    // Mercury
    let mercury = spawn_body(&mut commands, Body::new(0.387 * AU, 0.0, 8.0, 0.330e24, Color::WHITE, false));
    bodies.push(mercury);

    // Venus
    let venus = spawn_body(&mut commands, Body::new(0.723 * AU, 0.0, 14.0, 4.86e24, Color::WHITE, false));
    bodies.push(venus);

    // Earth
    let earth = spawn_body(&mut commands, Body::new(-1.0 * AU, 0.0, 16.0, 5.9742e24, Color::WHITE, false));
    bodies.push(earth);

    // Mars
    let mars = spawn_body(&mut commands, Body::new(-1.524 * AU, 0.0, 12.0, 6.39e23, Color::WHITE, false));
    bodies.push(mars);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_bodies(mut query: Query<(&mut Transform, &Body)>) {
    for (mut transform, body) in query.iter_mut() {
        let scaled_position = body.position * SCALE;
        transform.translation = scaled_position.extend(0.0);
    }
}
