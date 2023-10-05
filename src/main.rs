use std::ops::Range;

use crate::{
    core::viewport::{ Viewport, ViewportManager },
    geometry::particle::{ Acceleration, BodyBundle, LastPos, Mass },
};

use bevy::{
    prelude::*,
    window::{ PresentMode, Window, WindowPlugin, WindowTheme },
    DefaultPlugins,
};
mod bevy_runner;
use rand::{ rngs::StdRng, thread_rng, Rng, SeedableRng };

mod core;
mod geometry;
mod logger;

const GRAVITY_CONSTANT: f32 = 0.001;
const DELTA_TIME: f32 = 0.01;

fn main() {
    logger::logger_setup();
    bevy::prelude::App
        ::new()
        .insert_resource(ViewportManager {
            widescreen: Viewport::new(1920.0, 1080.0),
            vertical: Viewport::new(1080.0, 1920.0),
            square: Viewport::new(800.0, 800.0),
        })
        .insert_resource(FixedTime::new_from_secs(DELTA_TIME))
        .insert_resource(AmbientLight {
            brightness: 0.03,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "3d-render-app".into(),
                    resolution: Viewport::new(800.0, 800.0).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // mode: WindowMode::Fullscreen,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            // WireframePlugin,
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, geometry::frame::draw)
        .add_systems(FixedUpdate, (interact_bodies, integrate))
        // .add_systems(Update, (on_resize_system, toggle_resolution, setup_ui))
        // .add_systems(Update, animate_cube)
        .run();
}

#[derive(Component, Deref)]
struct Velocity(Vec2);

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    viewport_manager: Res<ViewportManager>
) {
    info!("startup");

    /* This system creates the primary frame */
    info!("creating-main-frame");
    let plane_size = 10.0;
    let frame_thickness = 6;
    let frame_start_position = Vec3::new(0.0, 0.0, 0.0);
    let cube_size = 0.1;
    let cube_color = Color::rgb(0.98, 0.98, 0.96);
    commands.spawn(geometry::frame::Frame {
        plane_size,
        thickness: frame_thickness,
        cube_size,
        cube_color,
        start_position: frame_start_position,
    });

    /* This system shows how to calculate the camera position based on the frame size and the fov */
    info!("creating-frame-camera");
    let aspect_ratio = viewport_manager.default().aspect_ratio();
    let [plane_size_x, plane_size_y]: [f32; 2] = viewport_manager
        .default()
        .aspect_scaling(plane_size);
    let fov = 45.0;
    let c = plane_size_x / 2.0;
    let beta: f32 = fov / 2.0;
    let z = c * (1.0 + 1.0 / beta.tan()) - c + (frame_thickness as f32) * cube_size;
    commands.spawn(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov,
            near: 0.1,
            far: 1000.0,
            aspect_ratio,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, z).looking_at(frame_start_position, Vec3::Y),
        ..default()
    });

    /* This system shows how to calculate the light position based on the frame size */
    info!("creating-frame-light");
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, plane_size_y / 2.0, z / 4.0),
        ..default()
    });

    /* This system creates the particles in the frame */
    info!("creating-particles");
    let particle_count = 128;
    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 3,
        }).unwrap()
    );
    let mut rng = thread_rng();
    let color_range = 0.5..1.0;
    let vel_range: [Range<f32>; 3] = [
        0.0..plane_size_x,
        0.0..plane_size_y,
        0.0..cube_size * (frame_thickness as f32),
    ];

    for _ in 0..particle_count {
        let radius: f32 = rng.gen_range(0.1..0.7);
        let mass_value = radius.powi(3) * 10.0;

        let position =
            Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0)
            ).normalize() *
            rng.gen_range(0.2f32..1.0).cbrt() *
            15.0;

        commands.spawn(BodyBundle {
            pbr: PbrBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(radius),
                    ..default()
                },
                mesh: mesh.clone(),
                material: materials.add(
                    Color::rgb(
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone())
                    ).into()
                ),
                ..default()
            },
            mass: Mass(mass_value),
            acceleration: Acceleration(Vec3::ZERO),
            last_pos: LastPos(
                position -
                    Vec3::new(
                        rng.gen_range(vel_range[0].clone()),
                        rng.gen_range(vel_range[1].clone()),
                        rng.gen_range(vel_range[2].clone())
                    ) *
                        DELTA_TIME
            ),
        });
    }

    info!("end-startup");
}

fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
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

fn integrate(mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
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
