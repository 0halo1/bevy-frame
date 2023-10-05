// geometry/frame.rs

use bevy::prelude::{
    shape, Assets, Color, Commands, Component, Mesh, PbrBundle, Query, Res, ResMut,
    StandardMaterial, Transform, Vec3,
};

use crate::core::viewport::ViewportManager;

#[derive(Component)]
pub struct Frame {
    pub plane_size: f32,
    pub thickness: usize,
    pub cube_size: f32,
    pub cube_color: Color,
    pub start_position: Vec3,
}

pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    viewport_manager: Res<ViewportManager>,
    mut query: Query<&Frame>,
) {
    /* Query the Frame */
    let frame = query.single_mut();

    /* Initialize the scene */
    let cube_size = frame.cube_size;
    let cube_color = frame.cube_color;
    let plane_size = frame.plane_size;
    let frame_thickness: usize = frame.thickness;
    let frame_start_position = frame.start_position;
    println!("frame_start_position: {:?}", frame_start_position);
    println!("frame_thickness: {}", frame_thickness);
    println!("frame_size: {}", plane_size);
    println!("cube_size: {}", cube_size);

    /* Scale frame_size by width and height for x and y */
    let [plane_size_x, plane_size_y] = viewport_manager.default().aspect_scaling(plane_size);

    /* Initialize the material & mesh */
    let cube_material = materials.add(cube_color.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());

    /* Intialize the structure */
    let [cube_count_x, cube_count_y] = [
        (plane_size_x / cube_size) as usize,
        (plane_size_y / cube_size) as usize,
    ];
    let [cube_offset_x, cube_offset_y] = [
        plane_size_x / 2.0 - cube_size / 2.0 + frame_start_position.x,
        plane_size_y / 2.0 - cube_size / 2.0 + frame_start_position.y,
    ];

    println!("cube_offset_x: {}", cube_offset_x);
    println!("cube_offset_y: {}", cube_offset_y);

    /* Draw the first layer of cubes, iterate through the x and y axis and spawn a cube at each position*/
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

    /* Draw the second layer of cubes, iterate through the x and y axis and spawn a cube at each position */
    for z in 1..frame_thickness {
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

pub fn draw_particles() {
    // // let wireframe_cube_size = CUBE_SIZE * cube_count_x as f32 / 2.0;
    // let wireframe_cube_color = Color::rgb(0.0, 0.0, 0.0);
    // let wireframe_cube_material = materials.add(wireframe_cube_color.into());
    // let wireframe_cube_mesh = meshes.add(
    //     shape::Capsule {
    //         radius: CUBE_SIZE / 8.0,
    //         depth: CUBE_SIZE / 8.0,
    //         ..Default::default()
    //     }
    //     .into(),
    // );

    // // commands.spawn(PbrBundle {
    // //     mesh: wireframe_cube_mesh.clone(),
    // //     material: wireframe_cube_material.clone(),
    // //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    // //     ..Default::default()
    // // });

    // let points = [[
    //     vec3(-3., 2., 3.),
    //     vec3(3., 8., 3.),
    //     vec3(-3., 8., 1.5),
    //     vec3(2., 2., 1.2),
    // ]];

    // for z in 1..5 {
    //     for x in 1..cube_count_x {
    //         for y in 1..cube_count_y as usize {
    //             let pos_x = x as f32 * (plane_size_x - CUBE_SIZE) / (cube_count_x - 1) as f32
    //                 - plane_size_x / 2.0
    //                 + CUBE_SIZE / 2.0;
    //             let pos_y = y as f32 * (plane_size_y - CUBE_SIZE) / (cube_count_y - 1) as f32
    //                 - plane_size_y / 2.0
    //                 + CUBE_SIZE / 2.0;
    //             println!("pos_x: {}, pos_y: {}, z: {}", pos_x, pos_y, z);

    //             // if random less than 0.5 continue
    //             if rand::random::<f32>() < 0.5 {
    //                 continue;
    //             }

    //             // Make a CubicCurve
    //             // let bezier = Bezier::new(points).to_curve();

    //             commands.spawn((
    //                 PbrBundle {
    //                     mesh: wireframe_cube_mesh.clone(),
    //                     material: wireframe_cube_material.clone(),
    //                     transform: Transform::from_xyz(pos_x, pos_y, z as f32 * CUBE_SIZE),
    //                     ..Default::default()
    //                 },
    //                 // Curve(bezier),
    //             ));
    //         }
    //     }
    // }
}
