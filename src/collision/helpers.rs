
use ::FloatType;

use num_traits::Float;

/// Return the sign of a float (+1 or -1)
pub fn sign(a: FloatType) -> FloatType {
    if a > 0.0 {
        1.0
    } else if a < 0.0 {
        -1.0
    } else {
        0.0
    }
}


/// Returns true if a range contains a value
pub fn in_range<T: Float>(value: T, min: T, max: T) -> bool {
    min <= value && value <= max
}


/// Returns true if two ranges intersect
pub fn ranges_intersect<T: Float>(a_min: T, a_max: T, b_min: T, b_max: T) -> bool {
    a_min < b_max && b_min < a_max
}


/// Returns the overlap of two ranges
/// Format: [min, max]
pub fn range_overlap<T: Float>(a_min: T, a_max: T, b_min: T, b_max: T) -> Option<T> {
    if ranges_intersect(a_min, a_max, b_min, b_max) {
        let right = a_max - b_min;
        let left = b_max - a_min;

        if right < left {
            Some(right)
        } else {
            Some(-left)
        }
    } else {
        None
    }
}

