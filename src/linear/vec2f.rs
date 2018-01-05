
use std::ops::{Add, Sub, Mul, Div, Index};

#[derive(Copy, Clone, Debug)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}


impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }

    pub fn dot(self, other: Vec2f) -> f32 {
        (self.x * other.x + self.y * other.y)
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(self, other: Vec2f) -> f32 {
        (self - other).length()
    }

    pub fn abs(self) -> Vec2f {
        Vec2f {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

    pub fn normalized(self) -> Vec2f {
        let len = self.length();

        Vec2f {
            x: self.x / len,
            y: self.y / len
        }
    }
}


impl Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


impl Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vec2f> for f32 {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self * rhs.x,
            y: self * rhs.y
        }
    }
}


impl Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Div<Vec2f> for f32 {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self / rhs.x,
            y: self / rhs.y
        }
    }
}



impl From<[f32; 2]> for Vec2f {
    fn from(arr: [f32; 2]) -> Self {
        Vec2f {
            x: arr[0],
            y: arr[1]
        }
    }
}


impl Into<[f32; 2]> for Vec2f {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}



impl Index<usize> for Vec2f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,

            _ => panic!("Index out of range")
        }
    }
}
