use bevy::window::{PresentMode, WindowResized};
use bevy::{prelude::*, window::WindowTheme};
use std::f32::consts::PI;
const CUBE_COLOR: Color = Color::rgb(0.98, 0.98, 0.96);

use crate::app::{Frame, FrameManager, ResolutionText};
use crate::{CUBE_SIZE, PLANE_X_MODIFIER, PLANE_Y_MODIFIER};

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
    mut windows: Query<&mut Window>,
) {
    println!("cube_size: {}", CUBE_SIZE);

    let window = windows.single_mut();
    let window_width = window.width();
    let window_height = window.height();
    println!("window_width: {}", window_width);
    println!("window_height: {}", window_height);

    let plane_size_x = window_width / PLANE_X_MODIFIER;
    let plane_size_y = window_height / PLANE_Y_MODIFIER;
    println!("plane_size_x: {}", plane_size_x);
    println!("plane_size_y: {}", plane_size_y);

    let cube_material = materials.add(CUBE_COLOR.into());
    let cube_mesh = meshes.add(shape::Cube { size: CUBE_SIZE }.into());

    let cube_count_x = (plane_size_x / CUBE_SIZE) as usize;
    let cube_count_y = (plane_size_y / CUBE_SIZE) as usize;
    println!("cube_count_x: {}", cube_count_x);
    println!("cube_count_y: {}", cube_count_y);

    let cube_offset_x = plane_size_x / 2.0 - CUBE_SIZE / 2.0;
    let cube_offset_y = plane_size_y / 2.0 - CUBE_SIZE / 2.0;
    println!("cube_offset_x: {}", cube_offset_x);
    println!("cube_offset_y: {}", cube_offset_y);

    for x in 0..cube_count_x {
        for y in 0..cube_count_y {
            if x == 0 && y == 0 || x == cube_count_x - 1 && y == cube_count_y - 1 {
                println!(
                    "x: {}, y: {}",
                    x as f32 * CUBE_SIZE - cube_offset_x,
                    y as f32 * CUBE_SIZE - cube_offset_y
                );
            }
            commands.spawn(PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                transform: Transform::from_xyz(
                    x as f32 * CUBE_SIZE - cube_offset_x,
                    y as f32 * CUBE_SIZE - cube_offset_y,
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
                            x as f32 * CUBE_SIZE - cube_offset_x,
                            y as f32 * CUBE_SIZE - cube_offset_y,
                            z as f32 * CUBE_SIZE,
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

pub(crate) fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Calculate distance A from camera to plane based on distance B and plane size
    let window = windows.single_mut();
    let window_width = window.width();
    let plane_size_x = window_width / PLANE_X_MODIFIER;

    // variables
    let c = plane_size_x / 2.0; // 1.6 for square
    let beta = 20.0; // 45 always
    println!("c: {}", c);
    println!("beta: {}", beta);

    // let fov = 45.0 * PI / 180.0;
    let distance_a = c / (beta * PI / 180.0).tan();
    println!("distance_a: {}", distance_a);

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, distance_a).looking_at(Vec3::ZERO, Vec3::Y),
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
