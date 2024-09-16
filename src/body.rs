use bevy::prelude::*;

#[derive(Component)]
pub struct Body {
    pub position: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
}

impl Body {
    pub fn new(x: f32, y: f32, radius: f32, mass: f32, color: Color) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius,
            mass,
            color,
        }
    }
}
