
use cgmath;
use cgmath::{Vector3, Matrix4};
use cgmath::{Zero};


pub trait Camera {
    fn get_transform(&self) -> Matrix4<f32>;

    fn set_position(&mut self, position: Vector3<f32>);
}


#[derive(Copy, Clone)]
pub struct OrthographicCamera {
    transform: Matrix4<f32>,

    position: Vector3<f32>
}


impl Camera for OrthographicCamera {
    fn get_transform(&self) -> Matrix4<f32> {
        self.transform * self.calculate_view()
    }

    fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
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
            transform: cgmath::ortho(left, right, bottom, top, -1.0, 1.0),
            position: Vector3::zero()
        }
    }

    fn calculate_view(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(self.position);

        translation
    }
}
