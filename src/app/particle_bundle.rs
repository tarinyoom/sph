use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, Mesh2dHandle};

use crate::model::Particle;

use super::GameComponent;

#[derive(Bundle)]
pub struct ParticleBundle {
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub particle: GameComponent<Particle>,
}
