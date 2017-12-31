

pub trait View {
    // Return the translation and scaling respectively
    fn get_transformation(&self) -> ([f32; 2], [f32;2]);
}


#[derive(Copy, Clone)]
pub struct BoundedView {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}


impl View for BoundedView {
    fn get_transformation(&self) -> ([f32; 2], [f32; 2]) {
        let x = (self.left + self.right) / 2.0;
        let y = (self.top + self.bottom) / 2.0;

        let w = (self.right - self.left) / 2.0;
        let h = (self.top - self.bottom) / 2.0;

        ([-x, -y], [1.0 / w, 1.0 / h])
    }
}

impl Default for BoundedView {
    fn default() -> Self {
        BoundedView {
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: 1.0,
        }
    }
}

