use app::{App, Frame, FrameManager};

use crate::app::CubeManager;

mod app;
mod bevy_runner;
mod logger;

fn main() {
    logger::logger_setup();

    /* Create the 3D Render Application */
    let app: App = app::App::new(
        "3d-render-app",
        FrameManager {
            widescreen: Frame::new(1920.0, 1080.0),
            vertical: Frame::new(1080.0, 1920.0),
            square: Frame::new(1000.0, 1000.0),
        },
        CubeManager { size: 0.1 },
    );

    /* Run the 3D Render Application */
    println!("App Name: {}", app.app_name());
}
