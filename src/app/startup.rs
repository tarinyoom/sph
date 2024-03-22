use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::ThreadRng;

use super::{GameComponent, GameResource, ParticleBundle};

use crate::model::{generate_particle, Globals, Grid};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let globals = Globals::default();
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..64 {
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
        let bg = MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new((globals.radius / 2.0) as f32).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        };
        commands.spawn(p).with_children(|parent| {
            parent.spawn(bg);
        });
    }
    commands.insert_resource(GameResource { val: globals });
    commands.insert_resource(GameResource {
        val: Grid::<Entity>::new(),
    });
    commands.spawn(Camera2dBundle::default());
}
