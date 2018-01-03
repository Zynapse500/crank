

pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2]
}


impl Line {
    /// Create a new line
    pub fn new(start: [f32; 2], end: [f32; 2]) -> Line {
        Line { start, end }
    }


    /// Return a unit vector in the same direction as a line
    pub fn get_direction(&self) -> [f32; 2] {
        ::vec2_normalize(::vec2_sub(self.end, self.start))
    }


    /// Return the length of the line
    pub fn get_length(&self) -> f32 {
        ::vec2_length(::vec2_sub(self.end, self.start))
    }
}
