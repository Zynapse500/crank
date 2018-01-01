
pub trait View {
    /// Return the translation and scaling respectively
    fn get_transformation(&self) -> ([f32; 2], [f32; 2]);


    /// Maps a point in the range [-1, 1] to the view's bounds
    fn ndc_to_world(&self, p: [f32; 2]) -> [f32; 2] {
        let (translation, scale) = self.get_transformation();

        [
            p[0] / scale[0] - translation[0],
            p[1] / scale[1] - translation[1]
        ]
    }
}




/// Define the bounds of a view.
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


/// A view that is centered around a certain point and extends a certain size in each direction.
/// Positive size is right and up.
#[derive(Copy, Clone)]
pub struct CenteredView {
    pub center: [f32; 2],
    pub size: [f32; 2]
}

impl View for CenteredView {
    fn get_transformation(&self) -> ([f32; 2], [f32; 2]) {
        let x: f32 = self.center[0];
        let y: f32 = self.center[1];

        let w: f32 = (self.size[0]) / 2.0;
        let h: f32 = (self.size[1]) / 2.0;

        ([-x, -y], [1.0 / w, 1.0 / h])
    }
}

impl Default for CenteredView {
    fn default() -> Self {
        CenteredView {
            center: [0.0, 0.0],
            size: [2.0, 2.0]
        }
    }
}


