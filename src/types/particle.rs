use bevy::prelude::{Component, Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Particle {
    pub fn new(x0: f32, x1: f32, v0: f32, v1: f32) -> Self {
        Particle {
            position: Vec2::new(x0, x1),
            velocity: Vec2::new(v0, v1),
        }
    }
}
