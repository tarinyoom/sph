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

fn make_color(rho: f64) -> Color {
    // from 0 to 10
    let r = (rho as f32 * 4000.0).min(1.0);
    let gb = (1.0 - r) / 2.0;
    return Color::rgb(r, gb, gb);
}

pub fn update_transforms(
    mut particles: Query<(
        &mut Transform,
        &Handle<ColorMaterial>,
        &mut GameComponent<Particle>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut transform, color, p) in &mut particles {
        transform.translation = unwrap(&p.val);
        let color_mat = materials.get_mut(&*color).unwrap();
        color_mat.color = make_color(p.val.density);
        // 0, 1 ,1
        // 1, .5 ,.5
    }
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0] as f32, x[1] as f32, 0.0)
}
