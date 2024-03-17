use super::GameComponent;
use super::ParticleBundle;
use bevy::prelude::*;
use rand::prelude::ThreadRng;

pub fn make_setup<P>(
    gen: impl Fn(&mut ThreadRng) -> P,
) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>)
where
    P: Send + Sync + 'static,
{
    move |mut commands, mut meshes, mut materials| {
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
                particle: GameComponent { val: gen(&mut rng) },
            };
            commands.spawn(p);
        }
        commands.spawn(Camera2dBundle::default());
    }
}
