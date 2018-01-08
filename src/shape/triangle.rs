
use ::{Vector2};
use super::{Bounded, Rectangle};

pub struct Triangle {
    pub points: [Vector2; 3]
}

impl Triangle {
    pub fn new(a: Vector2, b: Vector2, c: Vector2) -> Triangle {
        Triangle {
            points: [a, b, c]
        }
    }
}


impl Bounded for Triangle {
    fn bounding_box(&self) -> Rectangle {
        let mut min = Vector2::zero();
        let mut max = Vector2::zero();

        for point in self.points.iter() {
            min[0] = min!(point[0], min[0]);
            min[1] = min!(point[1], min[1]);

            max[0] = max!(point[0], max[0]);
            max[1] = max!(point[1], max[1]);
        }

        Rectangle { min, max }
    }
}
