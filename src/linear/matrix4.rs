use std::ops;

use super::Vector3;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    data: [[f32; 4]; 4]
}


impl Matrix4 {
    /// Create a new matrix filled with zeroes
    pub fn zeroes() -> Self {
        Matrix4::from_raw([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    /// Create a new identity matrix
    pub fn identity() -> Self {
        Matrix4::from_raw([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Create from raw data
    pub fn from_raw(raw: [[f32; 4]; 4]) -> Self {
        Matrix4 {
            data: raw
        }
    }

    /// Return raw data
    pub fn to_raw(self) -> [[f32; 4]; 4] {
        self.data
    }


    /// Translate matrix
    pub fn translate(self, amount: Vector3) -> Self {
        let translation = Matrix4::from_raw([
            [1.0, 0.0, 0.0, amount.x],
            [0.0, 1.0, 0.0, amount.y],
            [0.0, 0.0, 1.0, amount.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        translation * self
    }


    /// Create a new orthographic projection matrix
    pub fn orthographic_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Matrix4::from_raw([
            [2.0 / (right - left), 0.0,                  0.0,                -(right + left) / (right - left)],
            [0.0,                  2.0 / (top - bottom), 0.0,                -(top + bottom) / (top - bottom)],
            [0.0,                  0.0,                 -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0,                  0.0,                  0.0,                1.0],
        ])
    }



}


impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut result = Matrix4::zeroes();

        for j in 0..4 {
            for i in 0..4 {
                for a in 0..4 {
                    result[(j, i)] += self[(j, a)] * rhs[(a, i)];
                }
            }
        }

        result
    }
}


impl ops::Index<(usize, usize)> for Matrix4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}


impl ops::IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}



/*
pub type Matrix4 = [[f32; 4]; 4];


impl Matrix for Matrix4 {
    const COLS: u32 = 4;
    const ROWS: u32 = 4;

    /// Create a new matrix filled with zeroes
    fn zeroes() -> Self {
        [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]
    }

    /// Create a new identity matrix
    fn identity() -> Self {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    *//*
    /// Matrix multiplication
    fn mul(&self, rhs: &Self) -> Self {
        let mut result = Matrix4::zeroes();

        for j in 0..4 {
            for i in 0..4 {
                for a in 0..4 {
                    result[j][i] += self[j][a] * rhs[a][i];
                }
            }
        }

        result
    }*//*
}


impl ProjectionMatrix for Matrix4 {
    /// Create a new orthographic projection matrix
    fn orthographic_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        [
            [2.0 / (right - left), 0.0,                  0.0,                -(right + left) / (right - left)],
            [0.0,                  2.0 / (top - bottom), 0.0,                -(top + bottom) / (top - bottom)],
            [0.0,                  0.0,                 -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0,                  0.0,                  0.0,                1.0],
        ]
    }
}


impl<M: Matrix<>> ops::Mul<M> for Matrix4 {
    type Output = Box<Matrix>;

    fn mul(self, rhs: M) -> Self::Output {
        let mut result = Matrix4::zeroes();

        for j in 0..4 {
            for i in 0..4 {
                for a in 0..4 {
                    result[j][i] += self[j][a] * rhs[a][i];
                }
            }
        }

        Box::new(result)
    }
}
*/
