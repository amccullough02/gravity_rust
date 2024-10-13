use bevy::prelude::*;
use bevy::color::palettes::basic::{YELLOW, WHITE};
use bevy_prototype_lyon::prelude::*;

use crate::constants::{G, SCALE, TIMESTEP};
use crate::Bodies;

#[derive(Component, Clone, PartialEq)]
pub struct Body {
    pub position: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_sun: bool,
    pub distance_to_sun: f32,
    pub x_vel: f32,
    pub y_vel: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, radius: f32, mass: f32, is_sun: bool) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius,
            mass,
            is_sun,
            distance_to_sun: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
        }
    }
    fn attraction(&mut self, other: &Body) -> (f32, f32) {
        let other_x = other.position.x;
        let other_y = other.position.y;
        let distance_x = other_x - self.position.x;
        let distance_y = other_y - self.position.y;
        let distance = (distance_x + distance_y).sqrt();

        if other.is_sun {
            self.distance_to_sun = distance
        }

        let force = G * self.mass * other.mass / distance * distance;
        let theta = distance_y.atan2(distance_x);
        let force_x = theta.cos() * force;
        let force_y = theta.sin() * force;

        (force_x, force_y)
    }
    pub fn update_position(&mut self, bodies: &Res<Bodies>, query: &Query<&Body, Without<Body>>) {
        let mut total_fx = 0.0;
        let mut total_fy = 0.0;

        for &entity in &bodies.bodies {
            if let Ok(other_body) = query.get(entity) {
                if self.position == other_body.position {
                    continue;
                }

                let (fx, fy) = self.attraction(other_body);
                total_fx += fx;
                total_fy += fy;
            }
        }

        self.x_vel += total_fx / self.mass * TIMESTEP;
        self.y_vel += total_fy / self.mass * TIMESTEP;

        self.position.x += self.x_vel * TIMESTEP;
        self.position.y += self.y_vel * TIMESTEP;
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
