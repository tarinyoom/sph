use std::f64::consts::PI;

use itertools::izip;

use super::{Globals, Grid, Particle};

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

fn l2(u: &Vec<f64>, v: &Vec<f64>) -> f64 {
    izip!(u, v).map(|(x_u, x_v)| x_u * x_v).sum()
}

fn contribution<E: PartialEq>(a: (E, &Particle), b: (E, &Particle)) -> Vec<f64> {
    let (e_a, p_a) = a;
    let (e_b, p_b) = b;
    let scale = if e_a != e_b {
        let r = l2(&p_a.position, &p_b.position);
        let volume = PI * r.powf(8.0) / 4.0;
        let value = (r * r - 10.0 * 10.0).max(0.0);
        500.0 * value * value * value / volume
    } else {
        0.0
    };
    izip!(&p_a.position, &p_b.position)
        .map(|(a, b)| scale * (a - b))
        .collect()
}

fn add_vecs(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    izip!(a, b).map(|(a, b)| a + b).collect()
}

pub fn step<E: PartialEq + Copy>(
    e: E,
    p: &Particle,
    h: f64,
    g: &Globals,
    grid: &Grid<E>,
) -> Particle {
    let zero = p.position.iter().map(|_| 0.0).collect();
    let dv = grid
        .elems
        .iter()
        .map(|(e_, p_)| contribution((e, p), (*e_, p_)))
        .fold(zero, add_vecs);
    let (x, v) = izip!(&p.position, &p.velocity, &g.bounds_min, &g.bounds_max)
        .map(|(x, v, min, max)| step_1d(*x, *v, *min, *max, h))
        .unzip();
    Particle {
        position: x,
        velocity: add_vecs(v, dv),
    }
}
