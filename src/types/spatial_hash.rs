use bevy::prelude::{Entity, Resource, Vec2};

use crate::FluidParams;

#[derive(Resource)]
pub struct SpatialHash {
    particles: Vec<Vec<Entity>>,
    width: f32,
    dims: [usize; 2],
}

impl SpatialHash {
    pub fn new(fluid: &FluidParams) -> Self {
        let bounds = &fluid.bounds;
        let mut dims: [usize; 2] = [0, 0];
        for i in 0..2 {
            dims[i] = ((bounds.max[i] - bounds.min[i]) / fluid.r).ceil() as usize;
        }
        SpatialHash {
            particles: vec![vec![]; dims[0] * dims[1]],
            width: fluid.r,
            dims: dims,
        }
    }

    pub fn insert(&mut self, id: Entity, pos: &Vec2) {
        let i = (pos.x % self.width).floor() as usize;
        let j = (pos.y % self.width).floor() as usize;
		let h = self.hash(i, j);
		self.particles[h].push(id);
    }

    pub fn clear(&mut self) {
        self.particles = vec![vec![]; self.dims[0] * self.dims[1]];
    }

    fn hash(&self, i: usize, j: usize) -> usize {
        i * self.dims[1] + j
    }
}
