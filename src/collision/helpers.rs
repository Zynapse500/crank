

/// Return the sign of a float (+1 or -1)
pub fn sign(a: f32) -> f32 {
    if a > 0.0 {
        1.0
    } else {
        -1.0
    }
}


/// Returns true if a range contains a value
/// Format: [min, max]
pub fn range_contains(a: [f32; 2], value: f32) -> bool {
    a[0] <= value && value <= a[1]
}


/// Returns true if two ranges intersect
/// Format: [min, max]
pub fn ranges_intersect(a: [f32; 2], b: [f32; 2]) -> bool {
    a[0] <= b[1] && b[0] <= a[1]
}


/// Returns the overlap of two ranges
/// Format: [min, max]
pub fn ranges_overlap(a: [f32; 2], b: [f32; 2]) -> Option<f32> {
    if ranges_intersect(a, b) {
        let right = a[1] - b[0];
        let left = b[1] - a[0];

        if right < left {
            Some(right)
        } else {
            Some(-left)
        }
    } else {
        None
    }
}

