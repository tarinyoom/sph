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
    izip!(u, v)
        .map(|(x_u, x_v)| {
            let d = x_u - x_v;
            d * d
        })
        .sum()
}

fn contribution<E: PartialEq>(a: (E, &Particle), b: (E, &Particle), r: f64) -> f64 {
    let (e_a, p_a) = a;
    let (e_b, p_b) = b;
    if e_a != e_b {
        let d2 = l2(&p_a.position, &p_b.position);
        let volume = PI * r.powf(8.0) / 4.0;
        let value = (r * r - d2).max(0.0);
        value * value * value / volume
    } else {
        0.0
    }
}

pub fn step<E: PartialEq + Copy>(
    e: E,
    p: &Particle,
    h: f64,
    g: &Globals,
    grid: &Grid<E>,
) -> Particle {
    let rho = grid
        .elems
        .iter()
        .map(|(e_, p_)| contribution((e, p), (*e_, p_), g.radius))
        .fold(0.0, |a, b| a + b);
    let (x, v) = izip!(&p.position, &p.velocity, &g.bounds_min, &g.bounds_max)
        .map(|(x, v, min, max)| step_1d(*x, *v, *min, *max, h))
        .unzip();
    Particle {
        position: x,
        velocity: v,
        density: rho,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel() {
        let p1 = Particle {
            velocity: vec![10.0, 10.0],
            position: vec![0.0, 0.0],
            density: 0.0,
        };
        let p2 = Particle {
            velocity: vec![10.0, 10.0],
            position: vec![3.0, 3.0],
            density: 0.0,
        };
        let f = contribution::<u32>((1, &p1), (2, &p2), 6.0);
        assert_eq!(f, 0.004420970641441537);
    }
}
