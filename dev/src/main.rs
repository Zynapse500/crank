//! This is a program used for testing the API during it's development
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate crank;

mod frame_counter;
use frame_counter::FrameCounter;

use crank::{WindowHandle, KeyCode, UpdateInfo, Renderer, RenderBatch, BoundedView};
use crank::{Vec2f};


// Start a new game
fn main() {

    let a = Vec2f::new(3.0, 2.0);
    let b = Vec2f::new(1.0, 5.0);

    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);

    



    crank::run_game::<Game>(512, 512, "Crank").unwrap();
}


// A Game
struct Game {
    running: bool,
    window: WindowHandle,
    time: f32,

    batch: RenderBatch,

    frame_counter: FrameCounter,

    width: f32,
    height: f32,
    view: BoundedView
}


// Handle game loop
impl crank::Game for Game {
    fn setup(window: WindowHandle) -> Game {
        Game {
            running: true,
            window,
            time: 0.0,

            batch: RenderBatch::new(),

            frame_counter: FrameCounter::new(),

            width: 0.0,
            height: 0.0,
            view: BoundedView::default()
        }
    }

    fn update(&mut self, info: UpdateInfo) {
        self.time += info.dt;
        self.frame_counter.tick();

        self.draw();
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.submit_batch(&self.batch);
    }
    fn is_running(&self) -> bool {
        self.running
    }
}


// Handle window events
impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        // println!("Size: {:?}", (width, height));

        self.width = width as f32;
        self.height = height as f32;

        self.view.left = 0.0;
        self.view.right = self.width;
        self.view.top = 0.0;
        self.view.bottom = self.height;
    }

    fn key_pressed(&mut self, key: KeyCode) {
        match key {
            KeyCode::Escape => self.running = false,

            _ => ()
        }
    }


    fn key_released(&mut self, key: KeyCode) {
        match key {

            _ => ()
        }
    }


    fn mouse_moved(&mut self, x: i32, y: i32) {
        // println!("Cursor: ({}, {})", x, y);
    }
}


impl Game {
    fn draw(&mut self) {
        let v = Vec2f::from(0.0);

        self.batch.clear();

        self.batch.set_view(self.view);

        self.batch.set_fill_color([0.0, 1.0, 0.0, 1.0]);
        self.batch.draw_circle(v.into(), 15.0);


        self.batch.set_fill_color([1.0, 1.0, 0.0, 1.0]);
        self.batch.draw_circle([self.width, 0.0], 15.0);


        self.batch.set_fill_color([0.0, 1.0, 1.0, 1.0]);
        self.batch.draw_circle([0.0, self.height], 15.0);


        self.batch.set_fill_color([1.0, 1.0, 1.0, 1.0]);
        self.batch.draw_circle([self.width, self.height], 15.0);
    }
}










