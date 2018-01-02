//! This is a program used for testing the API during it's development
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate crank;
extern crate rand;

mod dev;

#[cfg(test)]
mod tests {
    ///////////////////////
    // Development tests //
    ///////////////////////
    use dev;

    #[test]
    fn rectangle_paint() {
        dev::rectangle_paint();
    }
}


