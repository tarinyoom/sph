mod simulation;
mod types;

use rand::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use simulation::update_particle;
use types::{Fluid, Particle};

const N: u32 = 5;
const RADIUS: f32 = 100.0;
const SPEED: f32 = 25.0;
const BOUNDS_MIN: [f32; 2] = [-100.0, -100.0];
const BOUNDS_MAX: [f32; 2] = [100.0, 100.0];

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
    let fluid = Fluid::new(N, RADIUS, SPEED, BOUNDS_MIN, BOUNDS_MAX);

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
    mut particle_data: Query<(Entity, &mut Transform, &mut Particle)>,
    timer: Res<Time>,
    fluid: Res<Fluid>,
) {
    let mut grid: Vec<Vec<Entity>> = vec![vec![]; fluid.grid_dims[0] * fluid.grid_dims[1]];
    let mut map: HashMap<Entity, Vec2> = HashMap::new();

    for (id, _, p) in &particle_data {
        let idx = fluid.grid_idx(&p);
        map.insert(id, p.position);
        grid[idx].push(id);
    }

    for (id, mut transform, mut p) in &mut particle_data {
        *p = update_particle(id, &p, &fluid, timer.delta_seconds(), &grid, &map);
        transform.translation = p.position.extend(0.0);
    }
}
