use crate::types::{Bounds, Particle};

pub fn update_particle(p: &Particle, b: &Bounds, h: f32) -> Particle {
    let mut x = p.position + p.velocity * h;
    let mut v = p.velocity;

    for i in 0..2 {
        (x[i], v[i]) = constrain_1d(x[i], v[i], b.min[i], b.max[i]);
    }

    Particle {
        position: x,
        velocity: v,
    }
}

fn constrain_1d(x: f32, v: f32, lower: f32, upper: f32) -> (f32, f32) {
    if x < lower {
        (lower, -v)
    } else if x > upper {
        (upper, -v)
    } else {
        (x, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec2;

    #[test]
    fn test_constrain_1d() {
        assert_eq!(constrain_1d(-3.0, -1.0, -2.0, 2.0), (-2.0, 1.0));
        assert_eq!(constrain_1d(3.0, -1.0, -2.0, 2.0), (2.0, 1.0));
        assert_eq!(constrain_1d(0.0, -1.0, -2.0, 2.0), (0.0, -1.0));
    }

    /// position, velocity before (4 flaots),
    /// then position, velocity after (4 floats),
    fn test_update_particle_helper(ps: &[f32; 8], bounds: &Bounds, h: f32) {
        let before = Particle {
            position: Vec2::new(ps[0], ps[1]),
            velocity: Vec2::new(ps[2], ps[3]),
        };

        let after = Particle {
            position: Vec2::new(ps[4], ps[5]),
            velocity: Vec2::new(ps[6], ps[7]),
        };

        assert_eq!(update_particle(&before, &bounds, h), after);
    }

    #[test]
    fn test_update_particle() {
        let bounds = Bounds {
            min: Vec2::new(-500.0, -300.0),
            max: Vec2::new(500.0, 300.0),
        };

        let no_collision = [0.0, 0.0, 2.0, 2.0, 0.2, 0.2, 2.0, 2.0];
        test_update_particle_helper(&no_collision, &bounds, 0.1);

        let collision = [-490.0, 290.0, -200.0, 200.0, -500.0, 300.0, 200.0, -200.0];
        test_update_particle_helper(&collision, &bounds, 0.1);
    }
}
