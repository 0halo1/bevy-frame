// use bevy::prelude::*;
mod bevy_runner;

use app::{App, Frame, FrameManager};
use bevy::prelude::info;

mod app;
mod logger;

// The plane is 200x200 units
const PLANE_X_MODIFIER: f32 = 100.0;
const PLANE_Y_MODIFIER: f32 = 100.0;

// Color of the cube
// const CUBE_COLOR: Color = Color::rgb(0.98, 0.98, 0.96);

// Size of the cube
const CUBE_SIZE: f32 = 0.125;

fn main() {
    logger::logger_setup();

    /* Create the 3D Render Application */
    let app: App = app::App::new(
        "3d-render-app",
        FrameManager {
            widescreen: Frame::new(1920.0, 1080.0),
            vertical: Frame::new(1080.0, 1920.0),
            square: Frame::new(640.0, 640.0),
        },
        0.125,
    );
    println!("App Name: {}", app.app_name());
    println!("Cube Size: {}", app.cube_size());
}
