


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: Copy + Clone> {
    x: T,
    y: T
}




impl<T: Copy + Clone> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}


//////////////////////////////
// Operate between two Vec2 //
//////////////////////////////

use std::ops::{Add, Sub, Mul, Div};

/// Add Vec2 and Vec2
impl<T: Copy + Clone + Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

/// Subtract Vec2 and Vec2
impl<T: Copy + Clone + Sub<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

/// Multiply Vec2 and Vec2
impl<T: Copy + Clone + Mul<Output = T>> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

/// Divide Vec2 and Vec2
impl<T: Copy + Clone + Div<Output = T>> Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}



/////////////////////////////////////
// Operate between Vec2 and scalar //
/////////////////////////////////////

/// Multiply scalar and Vec2
impl<T: Copy + Clone + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}



///////////////////////////////
// Convert from another type //
///////////////////////////////

/// Create Vec2 from a scalar
impl<T: Copy + Clone> From<T> for Vec2<T> {
    fn from(scalar: T) -> Self {
        Vec2 {
            x: scalar.clone(),
            y: scalar.clone()
        }
    }
}


/// Create Vec2 from an array
impl<T: Copy + Clone> From<[T; 2]> for Vec2<T> {
    fn from(array: [T; 2]) -> Self {
        Vec2 {
            x: array[0],
            y: array[1]
        }
    }
}



///////////////////////////////
// Convert into another type //
///////////////////////////////

/// Convert Vec2 to an array
impl<T: Copy + Clone> Into<[T; 2]> for Vec2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

/// Convert Vec2 to an array
impl<T: From<u8> + Copy + Clone> Into<[T; 3]> for Vec2<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, T::from(0)]
    }
}




///////////////////////////////
// Format Vec2 when printing //
///////////////////////////////

use std::fmt;

impl<T: Copy + Clone + fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}