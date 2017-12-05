use super::Vertex;

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}


impl Shape {
    pub fn new() -> Self {
        Shape {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }
}
