// geometry/frame.rs

use bevy::prelude::{
    shape, Assets, Commands, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Transform,
};

use crate::app::GeometryManager;

pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_manager: Res<GeometryManager>,
) {
    /* Initialize the scene */
    let cube_size = cube_manager.frame_cube_size;
    let cube_color = cube_manager.frame_cube_color;
    let frame_size: f32 = cube_manager.frame_size;
    let frame_thickness: usize = cube_manager.frame_thickness;
    let frame_start_position = cube_manager.frame_start_position;

    /* Initialize the material & mesh */
    let cube_material = materials.add(cube_color.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());

    /* Intialize the structure */
    let [cube_count_x, cube_count_y] = [
        (frame_size / cube_size) as usize,
        (frame_size / cube_size) as usize,
    ];
    let [cube_offset_x, cube_offset_y] = [
        frame_size / 2.0 - cube_size / 2.0 + frame_start_position.x,
        frame_size / 2.0 - cube_size / 2.0 + frame_start_position.y,
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
