use bevy::prelude::{Bundle, Color, Component, PbrBundle, Vec3};

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
