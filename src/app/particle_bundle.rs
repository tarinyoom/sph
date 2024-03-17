use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, Mesh2dHandle};

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
