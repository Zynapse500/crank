
use super::IntType;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, Index, IndexMut};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2i {
    pub x: IntType,
    pub y: IntType
}


impl Vector2i {
    /// Create a new vector with a x and y value
    pub fn new(x: IntType, y: IntType) -> Vector2i {
        Vector2i { x, y }
    }

    /// Create a vector filled with zeroes
    pub fn zero() -> Vector2i {
        Vector2i { x: 0, y: 0 }
    }

    /// Compute the dot product
    pub fn dot(self, other: Vector2i) -> IntType {
        (self.x * other.x + self.y * other.y)
    }

    /// Return a vector with both elements being the absolute counterparts of this vector
    pub fn abs(self) -> Vector2i {
        Vector2i {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

    /// Return the vector that is perpendicular to this vector
    pub fn perpendicular(&self) -> Vector2i {
        Vector2i {
            x: -self.y,
            y: self.x
        }
    }
}


impl Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign<Vector2i> for Vector2i {
    fn add_assign(&mut self, rhs: Vector2i) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign<Vector2i> for Vector2i {
    fn sub_assign(&mut self, rhs: Vector2i) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


impl Mul<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<IntType> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: IntType) -> Self::Output {
        Vector2i {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector2i> for IntType {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self * rhs.x,
            y: self * rhs.y
        }
    }
}

impl MulAssign<Vector2i> for Vector2i {
    fn mul_assign(&mut self, rhs: Vector2i) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<IntType> for Vector2i {
    fn mul_assign(&mut self, rhs: IntType) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


impl Div<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Div<IntType> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: IntType) -> Self::Output {
        Vector2i {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Div<Vector2i> for IntType {
    type Output = Vector2i;

    fn div(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self / rhs.x,
            y: self / rhs.y
        }
    }
}

impl DivAssign<Vector2i> for Vector2i {
    fn div_assign(&mut self, rhs: Vector2i) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<IntType> for Vector2i {
    fn div_assign(&mut self, rhs: IntType) {
        self.x /= rhs;
        self.y /= rhs;
    }
}



/// Negate the vector
impl Neg for Vector2i {
    type Output = Vector2i;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;

        self
    }
}




impl From<[IntType; 2]> for Vector2i {
    fn from(arr: [IntType; 2]) -> Self {
        Vector2i {
            x: arr[0],
            y: arr[1]
        }
    }
}


impl Into<[IntType; 2]> for Vector2i {
    fn into(self) -> [IntType; 2] {
        [self.x, self.y]
    }
}



impl Index<usize> for Vector2i {
    type Output = IntType;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,

            _ => panic!("Index out of range")
        }
    }
}


impl IndexMut<usize> for Vector2i {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,

            _ => panic!("Index out of range")
        }
    }
}

