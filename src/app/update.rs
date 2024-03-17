use super::GameComponent;
use bevy::prelude::{Entity, Query, Transform, Vec3};

pub fn make_update_system<P>(
    step: impl Fn(&P, f32) -> P,
    unwrap: impl Fn(&P) -> Vec3,
) -> impl Fn(Query<(Entity, &mut Transform, &mut GameComponent<P>)>)
where
    P: Send + Sync + 'static,
{
    move |mut particles| {
        for (_, mut transform, mut p) in &mut particles {
            p.val = step(&p.val, 0.1);
            transform.translation = unwrap(&p.val);
        }
    }
}
