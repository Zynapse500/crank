
use window;
use window::{KeyCode, MouseButton};

use renderer;
use renderer::frame::RenderFrame;

pub struct App {
    running: bool,


}

impl App {
    pub fn new() -> Self {
        App {
            running: true
        }
    }
}


impl window::WindowHandler for App {

    //////////////////////////////////////
    // Main loop /////////////////////////
    //////////////////////////////////////

    fn init(&mut self) {
        println!("Created new App");
    }

    fn update(&mut self, dt: f32) {
        // println!("Delta: {}", dt);
    }

    fn render(&mut self, frame: &mut RenderFrame) {
        frame.set_color(&[1.0, 0.1, 0.1, 1.0]);
        frame.draw_polygon(&[
            [0.0, 0.5],
            [0.5, -0.5],
            [-0.5, -0.5],
        ]);


        frame.set_color(&[0.1, 0.1, 1.0, 1.0]);
        frame.draw_polygon(&[
            [0.0, -0.5],
            [0.5, 0.5],
            [-0.5, 0.5],
        ]);

        frame.set_color(&[0.1, 1.0, 0.1, 1.0]);
        frame.draw_polygon(&[
            [0.0, -0.5],
            [0.25, 0.0],
            [0.0, 0.5],
            [0.0, 0.5],
            [-0.25, 0.0],
            [0.0, -0.5],
        ]);
    }

    //////////////////////////////////////
    // Events ////////////////////////////
    //////////////////////////////////////

    fn key_pressed(&mut self, key: KeyCode) {
        println!("Pressed: {:?}", key);


        match key {
            KeyCode::Escape => self.running = false,
            _ => ()
        }
    }

    fn key_released(&mut self, key: KeyCode) {
        println!("Released: {:?}", key);
    }

    fn mouse_pressed(&mut self, button: MouseButton) {
        println!("Pressed: {:?}", button);
    }

    fn mouse_released(&mut self, button: MouseButton) {
        println!("Released: {:?}", button);
    }

    fn resized(&mut self, width: u32, height: u32) {
        println!("Resized: ({}, {})", width, height);
    }

    fn closed(&mut self) { println!("Closed!"); }


    //////////////////////////////////////
    // State /////////////////////////////
    //////////////////////////////////////

    fn is_running(&self) -> bool {
        self.running
    }
}