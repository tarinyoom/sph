use bevy::prelude::Component;

#[derive(Component)]
pub struct GameComponent<T> {
    pub val: T,
}
