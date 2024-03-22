use std::f64::consts::PI;

use itertools::izip;
use rand::prelude::ThreadRng;
use rand::Rng;

use super::{Globals, Particle};

pub fn generate_particle(rng: &mut ThreadRng, g: &Globals) -> Particle {
    let x: Vec<f64> = izip!(&g.bounds_min, &g.bounds_max)
        .map(|(min, max)| (rng.gen::<f64>() - 0.5) * (max - min))
        .collect();
    let v: Vec<f64> = (&x)
        .iter()
        .map(|_| f64::sin(rng.gen::<f64>() * 2.0 * PI) * 5.0)
        .collect();
    Particle {
        position: x,
        velocity: v,
        density: 0.0,
    }
}
