
use num_traits::{NumOps, Float};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: Copy + Clone> {
    pub x: T,
    pub y: T
}



impl<T: Copy + Clone> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}



impl<T: Copy + Clone + NumOps<T, T> + Add<Output = T>> Vec2<T> {
    /// Dot multiplication
    pub fn dot(&self, a: Vec2<T>) -> T {
        self.x * a.x + self.y * a.y
    }


    /// Squared length/magnitude of vector
    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}


impl<T: Float> Vec2<T> {
    /// Length of vector
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }


    /// Round vector
    pub fn round(self) -> Self {
        Vec2 {
            x: self.x.round(),
            y: self.y.round()
        }
    }

    /// Floor vector
    pub fn floor(self) -> Self {
        Vec2 {
            x: self.x.floor(),
            y: self.y.floor()
        }
    }

    /// Round vector
    pub fn ceil(self) -> Self {
        Vec2 {
            x: self.x.ceil(),
            y: self.y.ceil()
        }
    }
}



//////////////////////////////
// Operate between two Vec2 //
//////////////////////////////

use std::ops::{Add, Sub, Mul, Div};

/// Add Vec2 and Vec2
impl<T: Copy + Clone + NumOps<T, T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

/// Subtract Vec2 and Vec2
impl<T: Copy + Clone + NumOps<T, T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

/// Multiply Vec2 and Vec2
impl<T: Copy + Clone + NumOps<T, T>> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

/// Divide Vec2 and Vec2
impl<T: Copy + Clone + NumOps<T, T>> Div<Vec2<T>> for Vec2<T> {
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
impl<T: Copy + Clone + NumOps<T, T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}



/////////////////////////////////
// Construct from another type //
/////////////////////////////////


impl From<Vec2<i32>> for Vec2<f32> {
    fn from(v: Vec2<i32>) -> Self {
        Vec2::new(v.x as f32, v.y as f32)
    }
}


/// Create Vec2 from a scalar
impl<T: Copy + Clone> From<T> for Vec2<T> {
    fn from(scalar: T) -> Self {
        Vec2 {
            x: scalar,
            y: scalar
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

/// Convert Vec2 to Vec2 of another type
/*impl<C, T: Copy + Clone + Into<C>> Into<Vec2<C>> for Vec2<T> {
    fn into(self) -> Vec2<C> {
        Vec2::new(self.x.into(), self.y.into())
    }
}*/

/// Convert Vec2 to an array
impl<T: Copy + Clone> Into<[T; 2]> for Vec2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

/// Convert Vec2 to an array
impl<T: Default + Copy + Clone> Into<[T; 3]> for Vec2<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, T::default()]
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