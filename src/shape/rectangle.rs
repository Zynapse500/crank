use ::collision::{Collide, Overlap, RayCast, Intersection, Sweep, Impact};
use ::collision::{range_contains, ranges_intersect, ranges_overlap, sign};
use super::{Line, Bounded};


#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub center: [f32; 2],
    pub size: [f32; 2],
}


impl Rectangle {
    /// Create a new rectangle
    pub fn new(center: [f32; 2], size: [f32; 2]) -> Rectangle {
        Rectangle { center, size }
    }


    /// Return true if rectangle contains the point
    pub fn contains(&self, point: [f32; 2]) -> bool {
        let (range_x, range_y) = self.get_ranges();
        range_contains(range_x, point[0]) && range_contains(range_y, point[1])
    }


    /// Return the range the rectangle occupies on each axis
    fn get_ranges(&self) -> ([f32; 2], [f32; 2]) {
        let sizes = [self.size[0] * 0.5, self.size[1] * 0.5];
        (
            [
                self.center[0] - sizes[0],
                self.center[0] + sizes[0]
            ],
            [
                self.center[1] - sizes[1],
                self.center[1] + sizes[1]
            ]
        )
    }
}


impl Bounded for Rectangle {
    fn bounding_box(&self) -> Rectangle {
        self.clone()
    }
}


impl Collide<Rectangle> for Rectangle {
    fn intersects(&self, other: &Rectangle) -> bool {
        // Overlap must occur on both x and y
        let (self_range_x, self_range_y) = self.get_ranges();
        let (other_range_x, other_range_y) = other.get_ranges();

        ranges_intersect(self_range_x, other_range_x) &&
            ranges_intersect(self_range_y, other_range_y)
    }

    fn overlap(&self, other: &Rectangle) -> Option<Overlap> {
        let (self_range_x, self_range_y) = self.get_ranges();
        let (other_range_x, other_range_y) = other.get_ranges();

        let overlap = (
            ranges_overlap(self_range_x, other_range_x),
            ranges_overlap(self_range_y, other_range_y)
        );

        if let (Some(overlap_x), Some(overlap_y)) = overlap {
            if overlap_x.abs() < overlap_y.abs() {
                Some(Overlap {
                    depth: overlap_x,
                    resolve: [-overlap_x, 0.0],
                })
            } else {
                Some(Overlap {
                    depth: overlap_y,
                    resolve: [0.0, -overlap_y],
                })
            }
        } else {
            None
        }
    }
}


impl RayCast for Rectangle {
    fn ray_intersection(&self, origin: [f32; 2], direction: [f32; 2]) -> Option<Intersection> {
        use std::f32::INFINITY;
        let (range_x, range_y) = self.get_ranges();
        let ranges = [range_x, range_y];

        /////////////////////////////////////////////////
        // Calculate the time it took to reach a bound //
        /////////////////////////////////////////////////

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

        // Calculate how long it took to enter and leave the box
        let final_times = [
            if times[0][0] > times[1][0] {times[0][0]} else {times[1][0]},
            if times[0][1] < times[1][1] {times[0][1]} else {times[1][1]}
        ];

        // Missed if we left the box before we entered on all axes
        if final_times[1] < final_times[0] {
            return None;
        }


        // Calculate the intersection points and normals
        let mut intersection = Intersection {
            time: final_times[0],
            point: ::vec2_add(origin, ::vec2_scale(final_times[0], direction)),
            normal: [0.0; 2],
        };

        // What side was hit?
        if times[0][0] > times[1][0] {
            // Left/right
            intersection.normal[0] = -sign(direction[0]);
        } else {
            // Top/bottom
            intersection.normal[1] = -sign(direction[1]);
        }

        Some(intersection)
    }
}


impl Sweep<Rectangle> for Rectangle {
    fn sweep(&self, path: [f32; 2], other: &Rectangle) -> Option<Impact> {
        let sum = Rectangle {
            center: other.center,
            size: [
                self.size[0] + other.size[0],
                self.size[1] + other.size[1]
            ],
        };

        let line = Line {
            start: self.center,
            end: [
                self.center[0] + path[0],
                self.center[1] + path[1],
            ],
        };

        match sum.line_intersection(&line) {
            Some(intersection) => Some(Impact::from(intersection)),
            None => None
        }
    }
}

