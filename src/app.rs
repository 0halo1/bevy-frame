// app

use bevy::{
    prelude::{default, Color, Component, PluginGroup, Resource, Startup, Update, Vec2, Vec3},
    window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme},
    DefaultPlugins,
};

use crate::{
    bevy_runner::{on_resize_system, setup_camera, setup_light, setup_ui, toggle_resolution},
    geometry, logger,
};

pub struct App {
    app_name: &'static str,
    frame_manager: ViewportManager,
    geometry_manager: GeometryManager,
}

impl App {
    pub fn new(
        app_name: &'static str,
        viewport_manager: ViewportManager,
        geometry_manager: GeometryManager,
    ) -> Self {
        let default_frame = *viewport_manager.default();
        let resolution: WindowResolution = (default_frame).into();
        logger::logger_setup();
        bevy::prelude::App::new()
            .insert_resource(viewport_manager)
            .insert_resource(geometry_manager)
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
            .add_systems(
                Startup,
                (geometry::frame::draw, setup_light, setup_camera, setup_ui),
            )
            .add_systems(Update, (on_resize_system, toggle_resolution))
            .run();
        Self {
            app_name,
            frame_manager: viewport_manager,
            geometry_manager: geometry_manager,
        }
    }

    pub fn app_name(&self) -> &'static str {
        self.app_name
    }
}

#[derive(Copy, Clone)]
pub struct Viewport {
    pub(crate) res_x: f32,
    pub(crate) res_y: f32,
}

impl Viewport {
    pub const fn new(res_x: f32, res_y: f32) -> Self {
        Self { res_x, res_y }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.res_y / self.res_x
    }

    pub fn aspect_scaling(&self, size: f32) -> [f32; 2] {
        return [size * 1.0 / self.aspect_ratio(), size * self.aspect_ratio()];
    }
}

impl Into<Vec2> for Viewport {
    fn into(self) -> Vec2 {
        Vec2::new(self.res_x, self.res_y)
    }
}

impl Into<WindowResolution> for Viewport {
    fn into(self) -> WindowResolution {
        WindowResolution::new(self.res_x, self.res_y)
    }
}

#[derive(Resource, Clone, Copy)]
pub struct ViewportManager {
    pub(crate) widescreen: Viewport, // 16:9
    pub(crate) vertical: Viewport,   // 9:16
    pub(crate) square: Viewport,     // 1:1
}

impl ViewportManager {
    /**
     * Returns the default frame, which is the square frame.
     * This is used when the user has not selected a resolution.
     */
    pub(crate) fn default(&self) -> &Viewport {
        &self.square
    }
}

#[derive(Copy, Clone)]
pub struct Frame {
    pub(crate) plane_size: f32,
    pub(crate) thickness: usize,
    pub(crate) cube_size: f32,
    pub(crate) cube_color: Color,
    pub(crate) start_position: Vec3,
}

#[derive(Resource, Copy, Clone)]
pub struct GeometryManager {
    pub(crate) frame: Frame,
}

impl GeometryManager {}
