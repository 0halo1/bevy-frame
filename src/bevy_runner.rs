use bevy::prelude::*;
use bevy::window::WindowResized;
use std::f32::consts::PI;

use crate::app::{Frame, GeometryManager, ViewportManager};

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
pub(crate) struct ResolutionText;

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
    cube_manager: Res<GeometryManager>,
    viewport_manager: Res<ViewportManager>,
) {
    /* This system shows how to calculate the camera position based on the frame size and the fov */
    let frame_size = cube_manager.frame_size;
    let frame_thickeness = cube_manager.frame_thickness;
    let frame_start_position = cube_manager.frame_start_position;
    let cube_size = cube_manager.frame_cube_size;

    let aspect_ratio = viewport_manager.default().aspect_ratio();
    let frame_size_x: f32 = viewport_manager.default().aspect_scaling(frame_size)[0];

    let fov = 45.0;
    let c = frame_size_x / 2.0;
    let beta: f32 = fov / 2.0;
    let z = c * (1.0 + 1.0 / beta.tan()) - c + frame_thickeness as f32 * cube_size;

    // Camera
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
}

/// This system shows how to request the window to a new resolution
pub(crate) fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ViewportManager>,
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
