use bevy::prelude::Resource;

use crate::Particle;

#[derive(Resource)]
pub struct Fluid {
    pub n: u32,
    pub radius: f32,
    pub speed: f32,
    pub grid_dims: [usize; 2],
    pub bounds_min: [f32; 2],
    pub bounds_max: [f32; 2],
}

impl Fluid {
    pub fn new(
        n: u32,
        radius: f32,
        speed: f32,
        bounds_min: [f32; 2],
        bounds_max: [f32; 2],
    ) -> Self {
        assert_ne!(radius, 0.0);

        let mut grid_dims: [usize; 2] = [0, 0];
        for i in 0..grid_dims.len() {
            grid_dims[i] = ((bounds_max[i] - bounds_min[i]) / radius + 1.0).floor() as usize;
        }

        Fluid {
            n: n,
            radius: radius,
            speed: speed,
            bounds_min: bounds_min,
            bounds_max: bounds_max,
            grid_dims: grid_dims,
        }
    }

    pub fn grid_idx(&self, p: &Particle) -> usize {
        let i = ((p.position.x - self.bounds_min[0]) / self.radius).floor() as usize;
        let j = ((p.position.y - self.bounds_min[1]) / self.radius).floor() as usize;
        i * self.grid_dims[1] + j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discretization() {
        let f_few = Fluid::new(0, 1.0, 0.0, [0.0, 0.0], [1.0, 2.0]);
        assert_eq!(f_few.grid_dims, [2, 3]);
        let f_few_frac = Fluid::new(0, 1.0, 0.0, [0.0, 0.0], [0.1, 0.9]);
        assert_eq!(f_few_frac.grid_dims, [1, 1]);
        let f_many = Fluid::new(0, 0.5, 0.0, [-5.0, -0.25], [1.0, 0.25]);
        assert_eq!(f_many.grid_dims, [13, 2]);
        let f_many_frac = Fluid::new(0, 0.312, 0.0, [-1.0, -1.0], [1.0, 1.0]);
        assert_eq!(f_many_frac.grid_dims, [7, 7]);
    }

    #[test]
    fn test_indexing() {
        let f = Fluid::new(0, 1.0, 0.0, [-1.0, -1.0], [1.0, 1.0]);

        let p_ur = Particle::new(0.9, 0.9, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ur), 4);
        let p_ul = Particle::new(-0.9, 0.9, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ul), 1);
        let p_ll = Particle::new(-0.9, -0.9, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ll), 0);
        let p_lr = Particle::new(0.9, -0.9, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_lr), 3);
    }

    #[test]
    fn test_corner_indexing() {
        let f = Fluid::new(0, 1.0, 0.0, [-1.0, -1.0], [1.0, 1.0]);

        let p_ur = Particle::new(1.0, 1.0, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ur), 8);
        let p_ul = Particle::new(-1.0, 1.0, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ul), 2);
        let p_ll = Particle::new(-1.0, -1.0, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_ll), 0);
        let p_lr = Particle::new(1.0, -1.0, 0.0, 0.0);
        assert_eq!(f.grid_idx(&p_lr), 6);
    }
}
