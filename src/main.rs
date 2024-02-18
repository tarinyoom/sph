mod simulation;
mod types;

use rand::prelude::*;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use simulation::update_particle;
use types::{Fluid, Particle};

const N: u32 = 1000;
const RADIUS: f32 = 5.0;
const SPEED: f32 = 25.0;
const X_MIN: f32 = -100.0;
const X_MAX: f32 = 100.0;
const Y_MIN: f32 = -100.0;
const Y_MAX: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_particles)
        .run();
}

fn random_particle(fluid: &Fluid) -> Particle {
    let mut rng = rand::thread_rng();

    let mut x = Vec2::ZERO;
    for i in 0..2 {
        x[i] = (rng.gen::<f32>() - 0.5) * (fluid.bounds_max[i] - fluid.bounds_min[i]);
    }

    let a = rng.gen::<f32>() * 2.0 * PI;
    let v = Vec2::from_angle(a) * fluid.speed;

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
    let bounds_min = [X_MIN, Y_MIN];
    let bounds_max = [X_MAX, Y_MAX];

    let mut grid_dims: [usize; 2] = [0, 0];
    for i in 0..grid_dims.len() {
        grid_dims[i] = ((bounds_max[i] - bounds_min[i]) / RADIUS) as usize;
    }

    let fluid = Fluid {
        n: N,
        radius: RADIUS,
        speed: SPEED,
        bounds_min: bounds_min,
        bounds_max: bounds_max,
        grid_dims: grid_dims,
    };

    for _ in 0..fluid.n {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            random_particle(&fluid),
        ));
    }

    commands.insert_resource(fluid);
    commands.spawn(Camera2dBundle::default());
}

fn update_particles(
    mut particles: Query<(Entity, &mut Transform, &mut Particle)>,
    timer: Res<Time>,
    fluid: Res<Fluid>,
) {
    for (_, mut transform, mut p) in &mut particles {
        *p = update_particle(&p, &fluid, timer.delta_seconds());
        transform.translation = p.position.extend(0.0);
    }
}
