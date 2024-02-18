use bevy::prelude::{Component, Resource, Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Resource)]
pub struct FluidParams {
    pub n: i32,
    pub speed: f32,
    pub bounds: Bounds,
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}
