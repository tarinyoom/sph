mod app;
mod sph;

use bevy::prelude::*;
use rand::prelude::ThreadRng;

use sph::Particle;

use app::{GameComponent, ParticleBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup::<Particle>)
        .add_systems(Update, update_particles)
        .run();
}

fn setup<P: Sync + Send + 'static + for<'a> From<&'a mut ThreadRng>>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let p: ParticleBundle<P> = ParticleBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            particle: GameComponent {
                val: (&mut rng).into(),
            },
        };
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
