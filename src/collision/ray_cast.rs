
use ::shape::Line;

/// Objects that can be tested for an intersection with a ray
pub trait RayCast {
    /// Return any intersection points with the ray
    fn ray_intersections(&self, origin: [f32; 2], direction: [f32; 2]) -> Vec<Intersection>;

    /// Return any intersection points with the line
    fn line_intersections(&self, line: &Line) -> Vec<Intersection> {
        // Calculate possible intersections
        let direction = line.get_direction();
        let ray_intersections = self.ray_intersections(line.start, direction);

        /////////////////////////////////////////////////
        // Filter intersections that don't lie on line //
        /////////////////////////////////////////////////

        // All valid intersections
        let mut line_intersections = Vec::new();

        // Find the maximum time of impact
        let max_time = line.get_length();

        // Check for valid times
        for intersection in ray_intersections.into_iter() {
            if 0.0 <= intersection.time_of_impact && intersection.time_of_impact <= max_time {
                line_intersections.push(intersection);
            }
        }

        line_intersections
    }
}


pub struct Intersection {
    pub time_of_impact: f32,
    pub point: [f32; 2],
    pub normal: [f32; 2]
}
