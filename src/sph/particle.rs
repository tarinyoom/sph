use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;

const BOUNDS_MIN: [f32; 2] = [-100.0, -100.0];
const BOUNDS_MAX: [f32; 2] = [100.0, 100.0];

pub struct Particle {
    pub position: Vec<f32>,
    pub velocity: Vec<f32>,
}

impl From<&mut ThreadRng> for Particle {
    fn from(rng: &mut ThreadRng) -> Self {
        let x: Vec<f32> = (BOUNDS_MIN)
            .into_iter()
            .zip(BOUNDS_MAX)
            .map(|(min, max)| (rng.gen::<f32>() - 0.5) * (max - min))
            .collect();
        let v: Vec<f32> = (&x)
            .into_iter()
            .map(|_| f32::sin(rng.gen::<f32>() * 2.0 * PI) * 10.0)
            .collect();
        Self {
            position: x,
            velocity: v,
        }
    }
}
