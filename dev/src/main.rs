//! This is a program used for testing the API during it's development
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate crank;

use crank::{WindowHandle, KeyCode, UpdateInfo, Renderer, RenderBatch, Vertex};


// Start a new game
fn main() {
    crank::run_game::<Game>(512, 512, "Crank").unwrap();
}


// A Game
struct Game {
    window: WindowHandle,

    time: f32
}


// Handle game loop
impl crank::Game for Game {
    fn setup(window: WindowHandle) -> Game {
        Game {
            window,
            time: 0.0
        }
    }

    fn update(&mut self, info: UpdateInfo) {
        self.time += info.dt;
    }

    fn render(&self, renderer: &mut Renderer) {

        let (mut y, mut x) = (self.time * 5.0).sin_cos();
        x *= 0.25;
        y *= 0.25;

        let batch = RenderBatch {
            vertices: vec![
                Vertex::new( [ x + 0.0, y + 0.5, 0.0]).with_color([1.0, 0.0, 0.0, 1.0]),
                Vertex::new( [ x + 0.5, y - 0.5, 0.0]).with_color([1.0, 1.0, 0.0, 1.0]),
                Vertex::new( [ x - 0.5, y - 0.5, 0.0]).with_color([1.0, 0.0, 1.0, 1.0]),
            ]
        };

        renderer.submit_batch(&batch);
    }
}


// Handle window events
impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        println!("Size: {:?}", (width, height));
    }

    fn key_pressed(&mut self, key: KeyCode) {
        println!("Pressed: {:?}", key);

        match key {
            KeyCode::F11 => {
                println!("Fullscreen!!!");
                self.window.toggle_fullscreen();
            }

            _ => ()
        }
    }

    fn key_released(&mut self, key: KeyCode) {
        println!("Released: {:?}", key);
    }
}
