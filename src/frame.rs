use bevy::prelude::Vec2;

pub(crate) struct Frame {
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
