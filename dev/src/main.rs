
extern crate crank;


/// This is a program used for testing the API during it's development

use crank::window::WindowHandle;
use crank::window::KeyCode;

fn main() {
    println!("Hello World!");

    crank::create_game::<Game>(512, 512, "Crank");
}



struct Game {
    window: WindowHandle
}


impl crank::Game for Game {
    fn setup(window: WindowHandle) -> Game {
        Game {
            window
        }
    }
}

impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: usize, height: usize) {
        unimplemented!()
    }

    fn key_pressed(&mut self, key: KeyCode) {
        println!("Pressed: {:?}", key);
    }

    fn key_released(&mut self, key: KeyCode) {
        println!("Released: {:?}", key);
    }
}
