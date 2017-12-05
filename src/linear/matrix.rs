
#[derive(Copy, Clone)]
pub struct Matrix4(pub [[f32; 4]; 4]);


impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }


    pub fn orthographic_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Matrix4([
            [2.0 / (right - left), 0.0,                  0.0,                -(right + left) / (right - left)],
            [0.0,                  2.0 / (top - bottom), 0.0,                -(top + bottom) / (top - bottom)],
            [0.0,                  0.0,                 -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0,                  0.0,                  0.0,                1.0],
        ])
    }
}
