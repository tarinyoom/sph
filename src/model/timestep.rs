use itertools::izip;

use super::{Globals, Particle};

fn step_1d(x: f64, v: f64, lower: f64, upper: f64, h: f64) -> (f64, f64) {
    let x = x + h * v;
    if x < lower {
        (lower, -v)
    } else if x > upper {
        (upper, -v)
    } else {
        (x, v)
    }
}

pub fn step(p: &Particle, h: f64, g: &Globals) -> Particle {
    let (x, v) = izip!(&p.position, &p.velocity, &g.bounds_min, &g.bounds_max)
        .map(|(x, v, min, max)| step_1d(*x, *v, *min, *max, h))
        .unzip();
    Particle {
        position: x,
        velocity: v,
    }
}
