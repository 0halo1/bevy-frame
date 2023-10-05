use crate::core::viewport::{Viewport, ViewportManager};

use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
    DefaultPlugins,
};
mod bevy_runner;

mod core;
mod geometry;
mod logger;

fn main() {
    logger::logger_setup();
    bevy::prelude::App::new()
        .insert_resource(ViewportManager {
            widescreen: Viewport::new(1920.0, 1080.0),
            vertical: Viewport::new(1080.0, 1920.0),
            square: Viewport::new(800.0, 800.0),
        })
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
        .add_systems(Update, geometry::frame::draw)
        // .add_systems(Update, (on_resize_system, toggle_resolution, setup_ui))
        // .add_systems(Update, animate_cube)
        .run();
}

fn startup(mut commands: Commands, viewport_manager: Res<ViewportManager>) {
    warn!("startup");

    /* This system creates the primary frame */
    warn!("creating-main-frame");
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
    warn!("creating-frame-camera");
    let aspect_ratio = viewport_manager.default().aspect_ratio();
    let [plane_size_x, plane_size_y]: [f32; 2] =
        viewport_manager.default().aspect_scaling(plane_size);
    let fov = 45.0;
    let c = plane_size_x / 2.0;
    let beta: f32 = fov / 2.0;
    let z = c * (1.0 + 1.0 / beta.tan()) - c + frame_thickness as f32 * cube_size;
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
    warn!("creating-frame-light");
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, plane_size_y / 2.0, z / 4.0),
        ..default()
    });

    warn!("end-startup");
}
