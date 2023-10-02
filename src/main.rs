//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (1000.0, 1000.0).into(),
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
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (setup, setup_light, setup_camera))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(1000.0, 1000.0);

    let cube_size = 0.125;
    println!("cube_size: {}", cube_size);

    let cube_color = Color::rgb(0.98, 0.98, 0.96);
    let cube_material = materials.add(cube_color.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());
    let plane_size_x = 5.0;
    let plane_size_y = 10.0;

    let cube_count_x = (plane_size_x / cube_size) as usize;
    let cube_count_y = (plane_size_y / cube_size) as usize;
    println!("cube_count: {}", cube_count_x);

    let cube_offset_x = plane_size_x / 2.0 - cube_size / 2.0;
    let cube_offset_y = plane_size_y / 2.0 - cube_size / 2.0;
    println!("cube_offset_x: {}", cube_offset_x);
    println!("cube_offset_y: {}", cube_offset_y);

    for x in 0..cube_count_x {
        for y in 0..cube_count_y {
            commands.spawn(PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                transform: Transform::from_xyz(
                    x as f32 * cube_size - cube_offset_x,
                    y as f32 * cube_size - cube_offset_y,
                    0.0,
                ),
                ..Default::default()
            });
        }
    }

    // draw another layer of cubes on top of the first layer but only on the edges, iterate this 3 times
    for z in 1..6 {
        for x in 0..cube_count_x {
            for y in 0..cube_count_y {
                if x == 0 || x == cube_count_x - 1 || y == 0 || y == cube_count_y - 1 {
                    commands.spawn(PbrBundle {
                        mesh: cube_mesh.clone(),
                        material: cube_material.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * cube_size - cube_offset_x,
                            y as f32 * cube_size - cube_offset_y,
                            z as f32 * cube_size,
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn setup_light(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 6.0, 4.0),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 6.75 * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
