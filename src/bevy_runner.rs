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
            aspect_ratio: window.height() / window.width(),
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
