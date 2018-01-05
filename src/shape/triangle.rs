
pub struct Triangle {
    pub points: [[f32; 2]; 3]
}

impl Triangle {
    pub fn new(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> Triangle {
        Triangle {
            points: [a, b, c]
        }
    }
}
