use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Fluid {
    pub n: u32,
    pub radius: f32,
    pub speed: f32,
    pub grid_dims: [usize; 2],
    pub bounds_min: [f32; 2],
    pub bounds_max: [f32; 2],
}
