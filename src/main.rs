mod app;
mod model;

use bevy::prelude::*;

use app::{make_setup, make_update_system};
use model::{generate_particle, step, Globals, Particle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, make_setup::<Particle, Globals>(generate_particle))
        .add_systems(Update, make_update_system(step, unwrap))
        .run();
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0] as f32, x[1] as f32, 0.0)
}
