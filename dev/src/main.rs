//! This is a program used for testing the API during it's development
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate crank;

use crank::{WindowHandle, KeyCode, UpdateInfo, Renderer, RenderBatch, BoundedView};


// Start a new game
fn main() {
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
        self.batch.clear();

        self.batch.set_view(self.view);

        self.batch.set_fill_color([0.0, 1.0, 0.0, 1.0]);
        self.batch.draw_circle([0.0, 0.0], 15.0);


        self.batch.set_fill_color([1.0, 1.0, 0.0, 1.0]);
        self.batch.draw_circle([self.width, 0.0], 15.0);


        self.batch.set_fill_color([0.0, 1.0, 1.0, 1.0]);
        self.batch.draw_circle([0.0, self.height], 15.0);


        self.batch.set_fill_color([1.0, 1.0, 1.0, 1.0]);
        self.batch.draw_circle([self.width, self.height], 15.0);
    }
}












use std::time::Instant;

struct FrameCounter {
    start: Instant,
    frames: u32,
    fps: u32
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter {
            start: Instant::now(),
            frames: 0,
            fps: 0
        }
    }


    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.start;
        let elapsed_secs = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;

        self.frames += 1;

        if elapsed_secs > 0.5 {
            self.fps = (self.frames as f32 / elapsed_secs).round() as u32;
            self.start = now;
            self.frames = 0;

            println!("FPS: {}", self.fps);
        }
    }


    pub fn get_rate(&self) -> u32 {
        self.fps
    }
}
