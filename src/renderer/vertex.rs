

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2]
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex {
            position,
            color: [1.0, 1.0, 1.0, 1.0],
            tex_coord: [0.0, 0.0]
        }
    }

    pub fn with_color(self, color: [f32; 4]) -> Vertex {
        Vertex {
            color,
            .. self
        }
    }

    pub fn with_tex_coord(self, tex_coord: [f32; 2]) -> Vertex {
        Vertex {
            tex_coord,
            .. self
        }
    }
}
