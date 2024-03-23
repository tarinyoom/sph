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

fn kernel(a: &Particle, b: &Particle, r: f64) -> f64 {
    let d2 = l2(&a.position, &b.position);
    let volume = PI * r.powf(8.0) / 4.0;
    let value = (r * r - d2).max(0.0);
    value * value * value / volume
}

fn contribution<E: PartialEq>(a: (E, &Particle), b: (E, &Particle), r: f64) -> f64 {
    let (e_a, p_a) = a;
    let (e_b, p_b) = b;
    if e_a != e_b {
        kernel(p_a, p_b, r)
    } else {
        0.0
    }
}

fn density<E: PartialEq + Copy>(e: E, p: &Particle, grid: &Grid<E>, r: f64) -> f64 {
    grid.elems
        .iter()
        .map(|(e_, p_)| contribution((e, p), (*e_, p_), r))
        .fold(0.0, |a, b| a + b)
}

pub fn step<E: PartialEq + Copy>(
    e: E,
    p: &Particle,
    h: f64,
    g: &Globals,
    grid: &Grid<E>,
) -> Particle {
    let rho = density(e, p, grid, g.radius);
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
        let p1 = vec![0.0, 0.0].into();
        let p2 = vec![3.0, 3.0].into();
        let f = contribution((1, &p1), (2, &p2), 6.0);
        assert_eq!(f, 0.004420970641441537);
    }

    #[test]
    fn test_density() {
        let p1: Particle = vec![0.0, 0.0].into();
        let p2 = vec![1.0, 1.0].into();
        let p3 = vec![0.0, 1.0].into();
        let g = Grid {
            elems: vec![(1, p1.clone()), (2, p2), (3, p3)],
        };
        assert_eq!(density(1, &p1, &g, 2.0), 0.17407571900676053);
    }
}
