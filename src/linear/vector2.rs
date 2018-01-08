
use super::FloatType;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, Index, IndexMut};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: FloatType,
    pub y: FloatType
}


impl Vector2 {
    /// Create a new vector with a x and y value
    pub fn new(x: FloatType, y: FloatType) -> Vector2 {
        Vector2 { x, y }
    }

    /// Create a vector filled with zeroes
    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    /// Compute the dot product
    pub fn dot(self, other: Vector2) -> FloatType {
        (self.x * other.x + self.y * other.y)
    }

    /// Return the length/magnitude of the vector
    pub fn length(self) -> FloatType {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Return the distance between two vectors
    pub fn distance(self, other: Vector2) -> FloatType {
        (self - other).length()
    }

    /// Return a vector with both elements being the absolute counterparts of this vector
    pub fn abs(self) -> Vector2 {
        Vector2 {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }


    /// Return the direction (normal) of this vector
    pub fn normal(mut self) -> Vector2 {
        let len = self.length();

        self.x /= len;
        self.y /= len;

        self
    }

    /// Return the vector that is perpendicular to this vector
    pub fn perpendicular(&self) -> Vector2 {
        Vector2 {
            x: -self.y,
            y: self.x
        }
    }
}


impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<FloatType> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: FloatType) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector2> for FloatType {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<FloatType> for Vector2 {
    fn mul_assign(&mut self, rhs: FloatType) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Div<FloatType> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: FloatType) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Div<Vector2> for FloatType {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self / rhs.x,
            y: self / rhs.y
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<FloatType> for Vector2 {
    fn div_assign(&mut self, rhs: FloatType) {
        self.x /= rhs;
        self.y /= rhs;
    }
}



/// Negate the vector
impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;

        self
    }
}




impl From<[FloatType; 2]> for Vector2 {
    fn from(arr: [FloatType; 2]) -> Self {
        Vector2 {
            x: arr[0],
            y: arr[1]
        }
    }
}


impl Into<[FloatType; 2]> for Vector2 {
    fn into(self) -> [FloatType; 2] {
        [self.x, self.y]
    }
}



impl Index<usize> for Vector2 {
    type Output = FloatType;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,

            _ => panic!("Index out of range")
        }
    }
}


impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,

            _ => panic!("Index out of range")
        }
    }
}

