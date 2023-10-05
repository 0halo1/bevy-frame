use bevy::window::WindowResolution;

use bevy::prelude::{Component, Resource, Vec2};

#[derive(Component, Clone, Copy)]
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

#[derive(Resource)]
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
