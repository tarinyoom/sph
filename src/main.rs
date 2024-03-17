mod app;
mod model;

use bevy::prelude::*;

use app::{make_setup, make_update_system};
use model::{generate_particle, step, Particle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, make_setup(generate_particle))
        .add_systems(Update, make_update_system(step, unwrap))
        .run();
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0], x[1], 0.0)
}
