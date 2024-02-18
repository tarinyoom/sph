mod fluid;
mod types;

use fluid::update_particle;
use types::{Bounds, FluidParams, Particle};

use rand::prelude::*;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const DEFAULT_FLUID_PARAMS: FluidParams = FluidParams {
    n: 1000,
    speed: 25.0,
    bounds: Bounds {
        min: Vec2::new(-500.0, -300.0),
        max: Vec2::new(500.0, 300.0),
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DEFAULT_FLUID_PARAMS)
        .add_systems(Startup, setup)
        .add_systems(Update, update_particles)
        .run();
}

fn random_particle(fluid: &Res<FluidParams>) -> Particle {
    let mut rng = rand::thread_rng();

    let mut x = Vec2::ZERO;
    for i in 0..2 {
        x[i] = (rng.gen::<f32>() - 0.5) * (fluid.bounds.max[i] - fluid.bounds.min[i]);
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
    fluid: Res<FluidParams>,
) {
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

    commands.spawn(Camera2dBundle::default());
}

fn update_particles(
    mut cubes: Query<(&mut Transform, &mut Particle)>,
    timer: Res<Time>,
    fluid: Res<FluidParams>,
) {
    for (mut transform, mut p) in &mut cubes {
        *p = update_particle(&p, &fluid.bounds, timer.delta_seconds());
        transform.translation = p.position.extend(0.0);
    }
}
