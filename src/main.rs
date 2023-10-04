use app::{App, Viewport, ViewportManager};
use bevy::prelude::{Color, Vec2, Vec3};

use crate::app::{Frame, GeometryManager};

mod app;
mod bevy_runner;
mod geometry;
mod logger;

fn main() {
    /* Create the 3D Render Application */
    let app: App = app::App::new(
        "3d-render-app",
        ViewportManager {
            widescreen: Viewport::new(1920.0, 1080.0),
            vertical: Viewport::new(1080.0, 1920.0),
            square: Viewport::new(800.0, 800.0),
        },
        GeometryManager {
            frame: Frame {
                plane_size: 10.0,
                thickness: 6,
                cube_size: 0.1,
                cube_color: Color::rgb(0.98, 0.98, 0.96),
                start_position: Vec3::new(0.0, 0.0, 0.0),
            },
        },
    );

    /* Run the 3D Render Application */
    println!("App Name: {}", app.app_name());
}
