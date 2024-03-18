use bevy::prelude::*;
use rand::prelude::ThreadRng;

use super::{GameComponent, GameResource, ParticleBundle};

use crate::{generate_particle, Globals};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let globals = Globals::default();
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..100 {
        let p: ParticleBundle = ParticleBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            particle: GameComponent {
                val: generate_particle(&mut rng, &globals),
            },
        };
        commands.spawn(p);
    }
    commands.insert_resource(GameResource { val: globals });
    commands.spawn(Camera2dBundle::default());
}
