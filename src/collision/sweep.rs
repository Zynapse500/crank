

pub trait Sweep<S> {
    fn sweep(&self, path: [f32; 2], other: &S) -> Option<Impact>;
}


pub struct Impact {
    pub time: f32,
    pub normal: [f32; 2]
}

impl Impact {
    pub fn inverse(self) -> Impact {
        Impact {
            time: self.time,
            normal: [-self.normal[0], - self.normal[1]],
        }
    }
}