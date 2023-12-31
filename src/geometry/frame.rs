// geometry/frame.rs

use bevy::prelude::{
    shape, Assets, Commands, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Transform,
};

use crate::app::{GeometryManager, ViewportManager};

pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_manager: Res<GeometryManager>,
    viewport_manager: Res<ViewportManager>,
) {
    /* Initialize the scene */
    let cube_size = cube_manager.frame.cube_size;
    let cube_color = cube_manager.frame.cube_color;
    let plane_size = cube_manager.frame.plane_size;
    let frame_thickness: usize = cube_manager.frame.thickness;
    let frame_start_position = cube_manager.frame.start_position;
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
