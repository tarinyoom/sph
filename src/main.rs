use rand::prelude::*;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const NUM_PARTICLES: i32 = 1000;
const RADIUS: f32 = 1.0;
const SPEED: f32 = 25.0;
const MIN_BOUNDS: [f32; 2] = [-500.0, -300.0];
const MAX_BOUNDS: [f32; 2] = [500.0, 300.0];

#[derive(Component, Debug, PartialEq)]
struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_particles)
        .run();
}

fn random_particle() -> Particle {
    let mut rng = rand::thread_rng();

    let mut x = Vec2::ZERO;
    for i in 0..2 {
        x[i] = (rng.gen::<f32>() - 0.5) * (MAX_BOUNDS[i] - MIN_BOUNDS[i] - 2.0 * RADIUS);
    }

    let a = rng.gen::<f32>() * 2.0 * PI;
    let v = Vec2::from_angle(a) * SPEED;

    Particle {
        position: x,
        velocity: v,
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..NUM_PARTICLES {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            random_particle(),
        ));
    }

    commands.spawn(Camera2dBundle::default());
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

fn update_particle(p: &Particle, h: f32) -> Particle {
    let mut x = p.position + p.velocity * h;
    let mut v = p.velocity;

    for i in 0..2 {
        (x[i], v[i]) = constrain_1d(x[i], v[i], MIN_BOUNDS[i] + RADIUS, MAX_BOUNDS[i] - RADIUS);
    }

    Particle {
        position: x,
        velocity: v,
    }
}

fn update_particles(mut cubes: Query<(&mut Transform, &mut Particle)>, timer: Res<Time>) {
    for (mut transform, mut p) in &mut cubes {
        *p = update_particle(&p, timer.delta_seconds());
        transform.translation = p.position.extend(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constrain_1d() {
        assert_eq!(constrain_1d(-3.0, -1.0, -2.0, 2.0), (-2.0, 1.0));
        assert_eq!(constrain_1d(3.0, -1.0, -2.0, 2.0), (2.0, 1.0));
        assert_eq!(constrain_1d(0.0, -1.0, -2.0, 2.0), (0.0, -1.0));
    }

    /// position, velocity before, then position, velocity after, then timestep
    fn test_update_particle_helper(params: &[f32; 9]) {
        let before = Particle {
            position: Vec2::new(params[0], params[1]),
            velocity: Vec2::new(params[2], params[3]),
        };
        let after = Particle {
            position: Vec2::new(params[4], params[5]),
            velocity: Vec2::new(params[6], params[7]),
        };
        assert_eq!(update_particle(&before, 0.1), after);
    }

    #[test]
    fn test_update_particle() {
        let no_collision = [0.0, 0.0, 2.0, 2.0, 0.2, 0.2, 2.0, 2.0, 0.1];
        test_update_particle_helper(&no_collision);

        let collision = [
            -490.0, 290.0, -100.0, 100.0, -499.0, 299.0, 100.0, -100.0, 0.1,
        ];
        test_update_particle_helper(&collision);
    }
}
