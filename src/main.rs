mod app;
mod model;

use bevy::prelude::*;

use app::{startup, update_grid, update_particles, update_transforms};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            (
                update_grid,
                update_particles.after(update_grid),
                update_transforms.after(update_particles),
            ),
        )
        .run();
}
