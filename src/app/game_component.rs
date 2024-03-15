use bevy::prelude::Component;

#[derive(Component)]
pub struct GameComponent<T> {
    pub val: T,
}

impl<T: Default> Default for GameComponent<T> {
    fn default() -> Self {
        Self {
            val: Default::default(),
        }
    }
}
