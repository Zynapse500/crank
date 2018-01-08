
use ::{FloatType, Vector2};

pub trait Sweep<S> {
    fn sweep(&self, path: Vector2, other: &S) -> Option<Impact>;
}


#[derive(Debug)]
pub struct Impact {
    pub time: FloatType,
    pub normal: Vector2
}

impl Impact {
    pub fn inverse(self) -> Impact {
        Impact {
            time: self.time,
            normal: -self.normal,
        }
    }
}