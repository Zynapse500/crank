use ::shape::Rectangle;

/// For objects that can collide with each other
pub trait Collide<C> {
    /// Return true if two objects intersect each other
    fn intersects(&self, other: &C) -> bool;

    /// Return the overlap of two objects
    fn overlap(&self, other: &C) -> Option<Overlap>;
}


pub struct Overlap {
    /// The depth of the overlap
    pub depth: f32,

    /// How much and in what direction the objects have to move in order to not overlap anymore
    pub resolve: [f32; 2]
}
