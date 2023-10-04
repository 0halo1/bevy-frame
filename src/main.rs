use app::{App, Frame, FrameManager};
use bevy::prelude::Color;

use crate::app::CubeManager;

mod app;
mod bevy_runner;
mod geometry;
mod logger;

fn main() {
    /* Create the 3D Render Application */
    let app: App = app::App::new(
        "3d-render-app",
        FrameManager {
            widescreen: Frame::new(1920.0, 1080.0),
            vertical: Frame::new(1080.0, 1920.0),
            square: Frame::new(640.0, 640.0),
        },
        CubeManager {
            size: 0.1,
            color: Color::rgb(0.98, 0.98, 0.96),
        },
    );

    /* Run the 3D Render Application */
    println!("App Name: {}", app.app_name());
}
