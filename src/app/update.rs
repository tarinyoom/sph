use bevy::prelude::*;

use super::{GameComponent, GameResource};

use crate::model::{step, Globals, Grid, Particle};

pub fn update_grid(
    particles: Query<(Entity, &GameComponent<Particle>)>,
    mut grid: ResMut<GameResource<Grid<Entity>>>,
) {
    grid.val = Grid {
        elems: particles
            .iter()
            .map(|(id, comp)| (id, comp.val.clone()))
            .collect(),
    };
}

pub fn update_particles(
    mut particles: Query<(Entity, &mut GameComponent<Particle>)>,
    globals: Res<GameResource<Globals>>,
    time: Res<Time<Fixed>>,
    grid: Res<GameResource<Grid<Entity>>>,
) {
    let g = &globals.into_inner().val;
    for (e, mut p) in &mut particles {
        p.val = step(e, &p.val, (*time).delta_seconds_f64(), g, &grid.val);
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
