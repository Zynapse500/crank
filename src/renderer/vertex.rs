

#[derive(Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4]
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex {
            position,
            color: [1.0, 1.0, 1.0, 1.0]
        }
    }

    pub fn with_color(self, color: [f32; 4]) -> Vertex {
        Vertex {
            color,
            .. self
        }
    }
}
