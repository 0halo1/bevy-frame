// geometry/structural/frame.rs

use bevy::prelude::{
    shape, Assets, Commands, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Transform,
};

use crate::app::CubeManager;

/// set up a simple 3D scene
pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_manager: Res<CubeManager>,
) {
    let cube_size = cube_manager.size;
    let cube_color = cube_manager.color;

    let plane_size_x = 10.0;
    let plane_size_y = 10.0;

    let cube_material = materials.add(cube_color.into());
    let cube_mesh = meshes.add(shape::Cube { size: cube_size }.into());

    let cube_count_x = (plane_size_x / cube_size) as usize;
    let cube_count_y = (plane_size_y / cube_size) as usize;

    let cube_offset_x = plane_size_x / 2.0 - cube_size / 2.0;
    let cube_offset_y = plane_size_y / 2.0 - cube_size / 2.0;

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
