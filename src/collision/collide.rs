
use ::{Vector2, FloatType};


/// For objects that can collide with each other
pub trait Collide<C> {
    /// Return true if two objects intersect each other
    fn intersects(&self, other: &C) -> bool;

    /// Return the overlap of two objects
    fn overlap(&self, other: &C) -> Option<Overlap>;
}


#[derive(Debug)]
pub struct Overlap {
    /// The depth of the overlap
    pub depth: FloatType,

    /// How much and in what direction the objects have to move in order to not overlap anymore
    pub resolve: Vector2
}
