use ::collision::{Collide, Overlap, RayCast, Intersection, Sweep, Impact};
use ::collision::{in_range, ranges_intersect, range_overlap, sign};
use super::{Line, Bounded};
use ::{Vector2};
use ::renderer::view::View;

#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
}


impl Rectangle {
    /// Create a new rectangle centered around a point
    pub fn centered(center: Vector2, size: Vector2) -> Rectangle {
        let half_size = size.abs() / 2.0;
        Rectangle {
            min: center - half_size,
            max: center + half_size,
        }
    }


    /// Return true if rectangle contains the point
    pub fn contains(&self, point: Vector2) -> bool {
        in_range(point.x, self.min.x, self.max.x) &&
            in_range(point.y, self.min.y, self.max.y)
    }


    /// Calculate the center of the rectangle
    pub fn center(&self) -> Vector2 {
        // The center is the average point
        0.5 * (self.min + self.max)
    }

    /// Calculate the size of the rectangle
    pub fn size(&self) -> Vector2 {
        // The size is the distance from each bound on each axis
        (self.max - self.min).abs()
    }


    /// Translate rectangle by in a direction
    pub fn translate(&mut self, delta: Vector2) {
        self.min += delta;
        self.max += delta;
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            min: Vector2::new(-1.0, -1.0),
            max: Vector2::new(1.0, 1.0)
        }
    }
}


impl Bounded for Rectangle {
    fn bounding_box(&self) -> Rectangle {
        self.clone()
    }
}


impl View for Rectangle {
    fn get_transformation(&self) -> (Vector2, Vector2) {
        (-self.center(), 2.0 / self.size())
    }
}



impl Collide<Rectangle> for Rectangle {
    fn intersects(&self, other: &Rectangle) -> bool {
        ranges_intersect(self.min.x, self.max.x, other.min.x, other.max.x) &&
            ranges_intersect(self.min.y, self.max.y, other.min.y, other.max.y)
    }

    fn overlap(&self, other: &Rectangle) -> Option<Overlap> {
        let overlap = (
            range_overlap(self.min.x, self.max.x, other.min.x, other.max.x),
            range_overlap(self.min.y, self.max.y, other.min.y, other.max.y)
        );

        if let (Some(overlap_x), Some(overlap_y)) = overlap {
            if overlap_x.abs() < overlap_y.abs() {
                Some(Overlap {
                    depth: overlap_x,
                    resolve: Vector2::new(-overlap_x, 0.0),
                })
            } else {
                Some(Overlap {
                    depth: overlap_y,
                    resolve: Vector2::new(0.0, -overlap_y),
                })
            }
        } else {
            None
        }
    }
}


impl RayCast for Rectangle {
    fn ray_intersection(&self, origin: Vector2, direction: Vector2) -> Option<Intersection> {


        /////////////////////////////////////////////////
        // Calculate the time it took to reach a bound //
        /////////////////////////////////////////////////

        let inverse_direction = 1.0 / direction;

        // When did the ray intersect the min and max bounds
        let min_times = (self.min - origin) * inverse_direction;
        let max_times = (self.max - origin) * inverse_direction;

        // When did the ray enter and leave each slab
        let entry_times = Vector2::new(
            min!(min_times.x, max_times.x),
            min!(min_times.y, max_times.y),
        );
        let exit_times = Vector2::new(
            max!(min_times.x, max_times.x),
            max!(min_times.y, max_times.y),
        );

        // When did the ray cross the nearest and furthest bound
        let near_time = max!(entry_times.x, entry_times.y);
        let far_time = min!(exit_times.x, exit_times.y);


        /*
                // o + dt = p => dt = p - o => t = (p - o) / d
                let mut times = [[-INFINITY, INFINITY]; 2];
                for i in 0..2 {
                    if direction[i] > 0.0 {
                        let inv = 1.0 / direction[i];
                        times[i] = [
                            inv * (ranges[i][0] - origin[i]),
                            inv * (ranges[i][1] - origin[i])
                        ];
                    }  else if direction[i] < 0.0 {
                        let inv = 1.0 / direction[i];
                        times[i] = [
                            inv * (ranges[i][1] - origin[i]),
                            inv * (ranges[i][0] - origin[i])
                        ];
                    } else {
                        // If the ray is going along an axis, make sure it is facing the rectangle
                        if !range_contains(ranges[i], origin[i]) {
                            return None;
                        }
                    }
                }
        */

        // Missed if we left the box before we entered on all axes
        if far_time < near_time {
            return None;
        }


        // Calculate the intersection points and normals
        let mut intersection = Intersection {
            time: near_time,
            point: origin + near_time * direction,
            normal: Vector2::zero(),
        };

        // What side was hit?
        if entry_times.x > entry_times.y {
            // Left/right
            intersection.normal.x = -sign(direction.x);
        } else {
            // Top/bottom
            intersection.normal.y = -sign(direction.y);
        }

        Some(intersection)
    }
}


impl Sweep<Rectangle> for Rectangle {
    fn sweep(&self, path: Vector2, other: &Rectangle) -> Option<Impact> {
        // Combine the sizes of the rectangles: Minkowski addition with the origin in 'other'
        let sum = Rectangle::centered(
            other.center(),
            self.size() + other.size()
        );

        let line = Line {
            start: self.center(),
            end: self.center() + path,
        };

        match sum.line_intersection(&line) {
            Some(intersection) => Some(Impact::from(intersection)),
            None => None
        }
    }
}

