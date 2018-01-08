
use ::shape::Line;
use ::{FloatType, Vector2};

/// Objects that can be tested for an intersection with a ray
pub trait RayCast {
    /// Return the first intersection point with the ray
    fn ray_intersection(&self, origin: Vector2, direction: Vector2) -> Option<Intersection>;

    /// Return the first intersection point with the line
    fn line_intersection(&self, line: &Line) -> Option<Intersection> {
        // Calculate possible intersections
        let direction = line.get_delta();
        let ray_intersection = self.ray_intersection(line.start, direction);

        /////////////////////////////////////////////////
        // Filter intersections that don't lie on line //
        /////////////////////////////////////////////////

        let len = direction.length();
        if len > 0.0 {
            if let Some(intersection) = ray_intersection {
                // Add some tolerance for rounding errors
                if 0.0 <= intersection.time && intersection.time <= 1.0 {
                    Some(intersection)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub time: FloatType,
    pub point: Vector2,
    pub normal: Vector2
}
