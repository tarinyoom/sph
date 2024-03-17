use super::{GameComponent, GameResource};
use bevy::prelude::{Entity, Query, Res, Transform, Vec3};

pub fn make_update_system<P, G>(
    step: impl Fn(&P, f64, &G) -> P,
    unwrap: impl Fn(&P) -> Vec3,
) -> impl Fn(Query<(Entity, &mut Transform, &mut GameComponent<P>)>, Res<GameResource<G>>)
where
    P: Send + Sync + 'static,
    G: Send + Sync + 'static,
{
    move |mut particles, global_resource| {
        let g = &global_resource.into_inner().val;
        for (_, mut transform, mut p) in &mut particles {
            p.val = step(&p.val, 0.1, g);
            transform.translation = unwrap(&p.val);
        }
    }
}
