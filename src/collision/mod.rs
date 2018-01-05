

mod collide;
pub use self::collide::{Collide, Overlap};

mod ray_cast;
pub use self::ray_cast::{RayCast, Intersection};

mod sweep;
pub use self::sweep::{Sweep, Impact};

mod helpers;
pub use self::helpers::*;


impl From<Intersection> for Impact {
    fn from(intersection: Intersection) -> Self {
        Impact {
            time: intersection.time,
            normal: intersection.normal,
        }
    }
}
