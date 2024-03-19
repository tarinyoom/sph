mod app;
mod model;

use bevy::prelude::*;

use app::{startup, update_particles, update_transforms};
use model::{generate_particle, step, Globals, Particle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            (update_particles, update_transforms.after(update_particles)),
        )
        .run();
}
