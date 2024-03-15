use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, Mesh2dHandle};
use rand::prelude::ThreadRng;

use super::GameComponent;

#[derive(Bundle)]
pub struct ParticleBundle<P>
where
    P: Send + Sync + 'static,
{
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub particle: GameComponent<P>,
}

pub fn generate_particle_bundle<P: Send + Sync + for<'a> From<&'a mut ThreadRng>>(
    rng: &mut ThreadRng,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) -> ParticleBundle<P> {
    ParticleBundle {
        mesh: meshes.add(shape::Circle::new(2.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Default::default(),
        global_transform: Default::default(),
        visibility: Default::default(),
        inherited_visibility: Default::default(),
        view_visibility: Default::default(),
        particle: GameComponent { val: rng.into() },
    }
}
