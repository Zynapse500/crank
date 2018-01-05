
use num_traits::{Num, Float, Signed, abs};
use std::ops::Neg;


/// Vector addition
pub fn vec2_add<T: Copy + Num>(a: [T; 2], b: [T; 2]) -> [T; 2] {
    [a[0] + b[0], a[1] + b[1]]
}


/// Vector subtraction
pub fn vec2_sub<T: Copy + Num>(a: [T; 2], b: [T; 2]) -> [T; 2] {
    [a[0] - b[0], a[1] - b[1]]
}


/// Vector multiplication
pub fn vec2_mul<T: Copy + Num>(a: [T; 2], b: [T; 2]) -> [T; 2] {
    [a[0] * b[0], a[1] * b[1]]
}


/// Vector division
pub fn vec2_div<T: Copy + Num>(a: [T; 2], b: [T; 2]) -> [T; 2] {
    [a[0] / b[0], a[1] / b[1]]
}




/// Scale
pub fn vec2_scale<T: Copy + Num>(s: T, a: [T; 2]) -> [T; 2] {
    [s * a[0], s * a[1]]
}



/// Perpendicular of vector
pub fn vec2_perp<T: Copy + Num + Neg<Output = T>>(a: [T; 2]) -> [T; 2] {
    [-a[1], a[0]]
}



/// Dot product
pub fn vec2_dot<T: Copy + Num>(a: [T; 2], b: [T; 2]) -> T {
    a[0] * b[0] + a[1] * b[1]
}

/// Length
pub fn vec2_length<T: Copy + Float>(a: [T; 2]) -> T {
    (a[0] * a[0] + a[1] * a[1]).sqrt()
}

/// Distance
pub fn vec2_distance<T: Copy + Float>(a: [T; 2], b: [T; 2]) -> T {
    let d = [a[0] - b[0], a[1] - b[1]];
    (d[0] * d[0] + d[1] * d[1]).sqrt()
}

/// Normalize
pub fn vec2_normalize<T: Copy + Float>(a: [T; 2]) -> [T; 2] {
    vec2_scale(T::one() / vec2_length(a), a)
}


/// Absolute value
pub fn vec2_abs<T: Copy + Signed>(a: [T; 2]) -> [T; 2] {
    [abs(a[0]), abs(a[1])]
}
