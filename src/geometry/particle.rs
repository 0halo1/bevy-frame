use bevy::prelude::{ Bundle, Color, Component, PbrBundle, Vec3, GlobalTransform, Query, Transform };

use crate::{ GRAVITY_CONSTANT, DELTA_TIME };

#[derive(Component, Default)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct LastPos(pub Vec3);

#[derive(Component)]
pub struct Start(pub Vec3);

#[derive(Bundle, Default)]
pub struct BodyBundle {
    pub pbr: PbrBundle,
    pub mass: Mass,
    pub last_pos: LastPos,
    pub acceleration: Acceleration,
}

pub fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while
        let Some(
            [
                (Mass(m1), transform1, mut acc1),
                (Mass(m2), transform2, mut acc2),
            ],
        ) = iter.fetch_next()
    {
        let delta = transform2.translation() - transform1.translation();
        let distance_sq: f32 = delta.length_squared();

        let f = GRAVITY_CONSTANT / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

pub fn integrate(mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
    let dt_sq = DELTA_TIME * DELTA_TIME;
    for (mut acceleration, mut transform, mut last_pos) in &mut query {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)

        let new_pos = transform.translation * 2.0 - last_pos.0 + acceleration.0 * dt_sq;
        acceleration.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}
