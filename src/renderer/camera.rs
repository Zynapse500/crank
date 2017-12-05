
use ::linear::matrix::Matrix4;

pub trait Camera {
    fn get_transform(&self) -> [[f32; 4]; 4];
}


#[derive(Copy, Clone)]
pub struct OrthographicCamera {
    transform: Matrix4
}


impl Camera for OrthographicCamera {
    fn get_transform(&self) -> [[f32; 4]; 4] {
        self.transform.0
    }
}

impl Default for OrthographicCamera {
    fn default() -> OrthographicCamera {
        OrthographicCamera::new(-1.0, 1.0, 1.0, -1.0)
    }
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        OrthographicCamera {
            transform: Matrix4::orthographic_projection(left, right, bottom, top, -1.0, 1.0)
        }
    }
}
