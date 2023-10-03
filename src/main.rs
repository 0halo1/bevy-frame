use bevy::{
    prelude::*,
    window::{PresentMode, WindowResized, WindowTheme},
};

// The following constants are used to set the resolution of the window.
const WIDESCREEN: Vec2 = Vec2::new(1920.0, 1080.0);
const VERTICAL: Vec2 = Vec2::new(1080.0, 1920.0);
const SQUARE: Vec2 = Vec2::new(640.0, 640.0);

// The plane is 200x200 units
const PLANE_X_MODIFIER: f32 = 200.0;
const PLANE_Y_MODIFIER: f32 = 200.0;

// Color of the cube
const CUBE_COLOR: Color = Color::rgb(0.98, 0.98, 0.96);

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            widescreen: WIDESCREEN,
            vertical: VERTICAL,
            square: SQUARE,
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // title: "I am a window!".into(),
                    resolution: VERTICAL.into(),
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
        .add_systems(Startup, (setup, setup_light, setup_camera, setup_ui))
        .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    widescreen: Vec2, // 16:9
    vertical: Vec2,   // 9:16
    square: Vec2,     // 1:1
}

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
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
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();
    let window_width = window.width();
    let window_height = window.height();
    println!("window_width: {}", window_width);
    println!("window_height: {}", window_height);

    let plane_size_x = window_width / PLANE_X_MODIFIER;
    let plane_size_y = window_height / PLANE_Y_MODIFIER;
    println!("plane_size_x: {}", plane_size_x);
    println!("plane_size_y: {}", plane_size_y);

    let cube_size = 0.125;
    println!("cube_size: {}", cube_size);

    let cube_material = materials.add(CUBE_COLOR.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());

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

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.widescreen;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.vertical;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution.square;
        window.resolution.set(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}
