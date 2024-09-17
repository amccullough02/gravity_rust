use bevy::prelude::*;

use crate::constants::{AU, G, SCALE, TIMESTEP};

#[derive(Component, Clone)]
pub struct Body {
    pub position: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub is_sun: bool,
}

impl Body {
    pub fn new(x: f32, y: f32, radius: f32, mass: f32, color: Color, is_sun: bool) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius,
            mass,
            color,
            is_sun,
        }
    }
}

pub fn spawn_body(commands: &mut Commands, body: Body) -> Entity {
    println!("{}", body.position);
    let scaled_position = body.position * SCALE;
    println!("{}", scaled_position);

    commands.spawn((
        body.clone(),
        SpriteBundle {
            sprite: Sprite {
                color: body.color,
                custom_size: Some(Vec2::new(body.radius * 2.0, body.radius * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(scaled_position.extend(0.0)),
            ..default()
        },
    ))
    .id()
}
