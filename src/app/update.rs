use bevy::prelude::*;

use super::{GameComponent, GameResource};

use crate::{step, Globals, Particle};

pub fn update_particles(
    mut particles: Query<&mut GameComponent<Particle>>,
    globals: Res<GameResource<Globals>>,
    time: Res<Time<Fixed>>,
) {
    let g = &globals.into_inner().val;
    for mut p in &mut particles {
        p.val = step(&p.val, (*time).delta_seconds_f64(), g);
    }
}

pub fn update_transforms(mut particles: Query<(&mut Transform, &mut GameComponent<Particle>)>) {
    for (mut transform, p) in &mut particles {
        transform.translation = unwrap(&p.val);
    }
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0] as f32, x[1] as f32, 0.0)
}
