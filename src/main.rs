use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod fps_counter;
use fps_counter::{fps_counter_system, setup_fps_counter};

mod body;
use body::{Body, spawn_body};

mod constants;
use constants::AU;

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
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup_fps_counter, setup_simulation, setup_camera))
        .add_systems(Update, (fps_counter_system, update_bodies))
        .run();
}

#[derive(Resource)]
pub struct Bodies {
    pub bodies: Vec<Entity>
}

fn setup_simulation(mut commands: Commands) {
    
    let mut bodies = Vec::new();

    let sun = spawn_body(&mut commands, Body::new(0.0, 0.0, 30.0, 1.989e30, true));
    bodies.push(sun);

    let mercury = spawn_body(&mut commands, Body::new(0.387 * AU, 0.0, 8.0, 0.330e24, false));
    bodies.push(mercury);

    let venus = spawn_body(&mut commands, Body::new(0.723 * AU, 0.0, 14.0, 4.86e24, false));
    bodies.push(venus);

    let earth = spawn_body(&mut commands, Body::new(-1.0 * AU, 0.0, 16.0, 5.9742e24, false));
    bodies.push(earth);

    let mars = spawn_body(&mut commands, Body::new(-1.524 * AU, 0.0, 12.0, 6.39e23, false));
    bodies.push(mars);

    commands.insert_resource(Bodies { bodies });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_bodies(bodies: Res<Bodies>, mut query: Query<&mut Body>) {
    for entity in &bodies.bodies {
        if let Ok(mut body) = query.get_mut(*entity) {
            body.update_position(&bodies, &query); // Pass the immutable reference of query
        }
    }
}

