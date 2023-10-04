use bevy::{
    prelude::{default, Component, PluginGroup, Resource, Startup, Update, Vec2},
    window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme},
    DefaultPlugins,
};

use crate::bevy_runner::{
    on_resize_system, setup, setup_camera, setup_light, setup_ui, toggle_resolution,
};

pub struct App {
    app_name: &'static str,
}

impl App {
    pub fn new(
        app_name: &'static str,
        frame_manager: FrameManager,
        cube_manager: CubeManager,
    ) -> Self {
        let resolution: WindowResolution = (*frame_manager.default()).into();
        bevy::prelude::App::new()
            .insert_resource(frame_manager)
            .insert_resource(cube_manager)
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: app_name.into(),
                        resolution: resolution,
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
        Self { app_name }
    }

    pub fn app_name(&self) -> &'static str {
        self.app_name
    }
}

#[derive(Copy, Clone)]
pub struct Frame {
    pub(crate) res_x: f32,
    pub(crate) res_y: f32,
}

impl Frame {
    pub const fn new(res_x: f32, res_y: f32) -> Self {
        Self { res_x, res_y }
    }

    pub(crate) fn aspect_ratio(&self) -> f32 {
        self.res_x / self.res_y
    }
}

impl Into<Vec2> for Frame {
    fn into(self) -> Vec2 {
        Vec2::new(self.res_x, self.res_y)
    }
}

impl Into<WindowResolution> for Frame {
    fn into(self) -> WindowResolution {
        WindowResolution::new(self.res_x, self.res_y)
    }
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
pub(crate) struct ResolutionText;

#[derive(Resource)]
pub struct FrameManager {
    pub(crate) widescreen: Frame, // 16:9
    pub(crate) vertical: Frame,   // 9:16
    pub(crate) square: Frame,     // 1:1
}

#[derive(Resource)]
pub struct CubeManager {
    pub(crate) size: f32,
}

impl FrameManager {
    /**
     * Returns the default frame, which is the square frame.
     * This is used when the user has not selected a resolution.
     */
    pub(crate) fn default(&self) -> &Frame {
        &self.square
    }
}
