use rand::prelude::*;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const NUM_PARTICLES: i32 = 1000;
const RADIUS: f32 = 1.0;
const SPEED: f32 = 25.0;
const MIN_BOUNDS: [f32; 2] = [-500.0, -300.0];
const MAX_BOUNDS: [f32; 2] = [500.0, 300.0];

#[derive(Component)]
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
