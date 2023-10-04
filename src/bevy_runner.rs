use bevy::prelude::*;
use bevy::window::WindowResized;
use std::f32::consts::PI;
const CUBE_COLOR: Color = Color::rgb(0.98, 0.98, 0.96);

use crate::app::{CubeManager, Frame, FrameManager, ResolutionText};

// Spawns the UI
pub(crate) fn setup_ui(mut cmd: Commands) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font_size: 50.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ResolutionText,
        ));
    });
}

/// set up a simple 3D scene
pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_manager: Res<CubeManager>,
) {
    let cube_size = cube_manager.size;
    let plane_size_x = 10.0;
    let plane_size_y = 10.0;
    println!("plane_size_x: {}", plane_size_x);
    println!("plane_size_y: {}", plane_size_y);

    let cube_material = materials.add(CUBE_COLOR.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());

    let cube_count_x = (plane_size_x / cube_size) as usize;
    let cube_count_y = (plane_size_y / cube_size) as usize;
    println!("cube_count_x: {}", cube_count_x);
    println!("cube_count_y: {}", cube_count_y);

    let cube_offset_x = plane_size_x / 2.0 - cube_size / 2.0;
    let cube_offset_y = plane_size_y / 2.0 - cube_size / 2.0;
    println!("cube_offset_x: {}", cube_offset_x);
    println!("cube_offset_y: {}", cube_offset_y);

    for x in 0..cube_count_x {
        for y in 0..cube_count_y {
            if x == 0 && y == 0 || x == cube_count_x - 1 && y == cube_count_y - 1 {
                println!(
                    "x: {}, y: {}",
                    x as f32 * cube_size - cube_offset_x,
                    y as f32 * cube_size - cube_offset_y
                );
            }
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

pub(crate) fn setup_light(mut commands: Commands) {
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

pub(crate) fn setup_camera(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    cube_manager: Res<CubeManager>,
) {
    let cube_size = cube_manager.size;
    // Calculate distance A from camera to plane based on distance B and plane size
    let window = windows.single_mut();

    let plane_size_x = 10.0;
    let fov = 45.0;
    let c = plane_size_x / 2.0 - cube_size * 3.0; // must be 4.25
    let beta = fov / 2.0; // 45/2 always
    println!("c: {}", c);
    println!("beta: {}", beta);

    // let fov = 45.0 * PI / 180.0;
    let z = c / (beta * (PI / 180.0)).tan();
    println!("z: {}", z);

    // camera
    commands.spawn(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov,
            // near: 0.1,
            // far: 1000.0,
            aspect_ratio: window.width() / window.height(),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, z).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// This system shows how to request the window to a new resolution
pub(crate) fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<FrameManager>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let frame: Frame = resolution.widescreen;
        window.resolution.set(frame.res_x, frame.res_y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let frame: Frame = resolution.vertical;
        window.resolution.set(frame.res_x, frame.res_y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let frame: Frame = resolution.square;
        window.resolution.set(frame.res_x, frame.res_y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
pub(crate) fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}
