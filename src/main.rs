mod app;
mod sph;

use bevy::prelude::*;

use sph::Particle;

use app::{generate_particle_bundle, GameComponent, ParticleBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup::<Particle>)
        .add_systems(Update, update_particles)
        .run();
}

fn setup<P>(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let p: ParticleBundle<Particle> =
            generate_particle_bundle(&mut rng, &mut meshes, &mut materials);
        commands.spawn(p);
    }
    commands.spawn(Camera2dBundle::default());
}

fn update_particles(
    mut particle_data: Query<(Entity, &mut Transform, &mut GameComponent<Particle>)>,
) {
    for (_, mut transform, p) in &mut particle_data {
        let x = &p.val.position;
        transform.translation = Vec3::new(x[0], x[1], 0.0);
    }
}
