
use super::Rectangle;
use ::{Vector2, FloatType};

#[derive(Debug)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2
}


impl Line {
    /// Create a new line
    pub fn new(start: Vector2, end: Vector2) -> Line {
        Line { start, end }
    }


    /// Return a unit vector in the same direction as a line
    pub fn get_direction(&self) -> Vector2 {
        (self.end - self.start).normal()
    }


    /// Return the vector between the start and end of the line
    pub fn get_delta(&self) -> Vector2 {
        self.end - self.start
    }



    /// Return the length of the line
    pub fn get_length(&self) -> FloatType {
        (self.end - self.start).length()
    }


    /// Return the bounding box of the line
    pub fn bounding_box(&self) -> Rectangle {
        Rectangle::centered(
            (self.start + self.end) / 2.0, // At the middle of the line
            (self.start - self.end).abs()    // The extent of the line
        )
    }
}
