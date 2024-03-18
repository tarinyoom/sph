use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameResource<T> {
    pub val: T,
}
