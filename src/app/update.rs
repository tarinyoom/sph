use bevy::prelude::*;

use super::{GameComponent, GameResource};

use crate::{step, Globals, Particle};

pub fn update(
    mut particles: Query<(Entity, &mut Transform, &mut GameComponent<Particle>)>,
    mut globals: Res<GameResource<Globals>>,
) {
    let g = &globals.into_inner().val;
    for (_, mut transform, mut p) in &mut particles {
        p.val = step(&p.val, 0.1, g);
        transform.translation = unwrap(&p.val);
    }
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0] as f32, x[1] as f32, 0.0)
}
