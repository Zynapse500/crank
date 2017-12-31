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
    running: bool,
    window: WindowHandle,
    time: f32,

    n: u32
}


// Handle game loop
impl crank::Game for Game {
    fn setup(window: WindowHandle) -> Game {
        Game {
            running: true,
            window,
            time: 0.0,

            n: 10
        }
    }

    fn update(&mut self, info: UpdateInfo) {
        self.time += info.dt;
    }

    fn render(&self, renderer: &mut Renderer) {
        let mut batch = RenderBatch::new();


        for i in 0..self.n {
            let p =  1.0 - i as f32 / self.n as f32;

            batch.set_fill_color([1.0, p, 0.0, 1.0]);
            batch.draw_rectangle([-p / 2.0, -p / 2.0], [p, p]);
        }

        batch.set_fill_color([0.0, 1.0, 1.0, 1.0]);
        batch.draw_circle([0.0, 0.0], 0.05 * (2.0 as f32).sqrt());

        renderer.submit_batch(&batch);
    }
    fn is_running(&self) -> bool {
        self.running
    }
}


// Handle window events
impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        println!("Size: {:?}", (width, height));
    }

    fn key_pressed(&mut self, key: KeyCode) {
        match key {
            KeyCode::Escape => self.running = false,

            KeyCode::Up => self.n += 1,
            KeyCode::Down => if self.n > 1 {self.n -= 1},

            _ => ()
        }


        println!("N: {}", self.n);
    }


    fn key_released(&mut self, key: KeyCode) {
        match key {

            _ => ()
        }
    }
}
