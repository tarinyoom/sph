use bevy::prelude::{Component, Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
}
