use std::f32::consts::PI;

use itertools::izip;
use rand::prelude::ThreadRng;
use rand::Rng;

use super::{Globals, Particle};

pub fn generate_particle(rng: &mut ThreadRng, g: &Globals) -> Particle {
    let x: Vec<f32> = izip!(&g.bounds_min, &g.bounds_max)
        .map(|(min, max)| (rng.gen::<f32>() - 0.5) * (max - min))
        .collect();
    let v: Vec<f32> = (&x)
        .iter()
        .map(|_| f32::sin(rng.gen::<f32>() * 2.0 * PI) * 10.0)
        .collect();
    Particle {
        position: x,
        velocity: v,
    }
}
