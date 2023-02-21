use cgmath;

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    top_left: [f32; 2],
    scale: f32,
    pub n_iter: f32,
    pub time: f32,
    padding: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new([-2.0, 2.0], 0.003, 100.0)
    }
}

impl Camera {
    pub fn new(point: [f32; 2], scale: f32, n_iter: f32) -> Self {
        Self {
            top_left: point,
            scale,
            n_iter,
            time: 0.0,
            padding: 0.0,
        }
    }

    pub fn set_top_left(&mut self, top_left: [f32; 2]) {
        self.top_left = top_left;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn get_top_left(&self) -> [f32; 2] {
        self.top_left
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}
