use bevy::prelude::{Component, Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}
