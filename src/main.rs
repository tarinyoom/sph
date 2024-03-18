mod app;
mod model;

use bevy::prelude::*;

use app::{startup, update};
use model::{generate_particle, step, Globals, Particle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}
