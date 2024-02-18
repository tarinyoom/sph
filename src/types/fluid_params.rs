use bevy::prelude::{Resource, Vec2};

#[derive(Resource)]
pub struct FluidParams {
    pub n: i32,
    pub r: f32,
    pub speed: f32,
	pub bounds: Bounds,
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}
