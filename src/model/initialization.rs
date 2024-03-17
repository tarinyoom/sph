use super::{Globals, Particle};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;

pub fn generate_particle(rng: &mut ThreadRng, g: &Globals) -> Particle {
    let x: Vec<f32> = (g.bounds_min.clone())
        .into_iter()
        .zip(g.bounds_max.clone())
        .map(|(min, max)| (rng.gen::<f32>() - 0.5) * (max - min))
        .collect();
    let v: Vec<f32> = (&x)
        .into_iter()
        .map(|_| f32::sin(rng.gen::<f32>() * 2.0 * PI) * 10.0)
        .collect();
    Particle {
        position: x,
        velocity: v,
    }
}
