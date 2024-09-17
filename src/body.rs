use bevy::prelude::*;
use bevy::color::palettes::basic::{YELLOW, WHITE};
use bevy_prototype_lyon::prelude::*;

use crate::constants::{AU, G, SCALE, TIMESTEP};

#[derive(Component, Clone)]
pub struct Body {
    pub position: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_sun: bool,
}

impl Body {
    pub fn new(x: f32, y: f32, radius: f32, mass: f32, is_sun: bool) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius,
            mass,
            is_sun,
        }
    }
}

pub fn spawn_body(commands: &mut Commands, body: Body) -> Entity {
    let scaled_position = body.position * SCALE;

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: body.radius,
                        center: Vec2::ZERO,
                    }
                ),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(scaled_position.extend(0.0)),
                    ..default()
                },
                ..default()
            },
            Fill::color(if body.is_sun { YELLOW } else { WHITE }),
            Stroke::new(Color::BLACK, 1.0),
            body.clone(),
        ))
        .id()
}
