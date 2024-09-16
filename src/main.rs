use bevy::prelude::*;

mod fps_counter;

use fps_counter::{fps_counter_system, setup_fps_counter};

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
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, fps_counter_system)
        .run();
}