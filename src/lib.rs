// For window and OpenGL context creation
extern crate glutin;

// For offset_of!
#[macro_use]
extern crate memoffset;


/// Contains bindings for OpenGL
mod gl;

/// Things related to a Game
pub mod game;

pub use game::{Game, UpdateInfo};

/// Things related to Rendering
pub mod renderer;
pub use renderer::{Renderer, RenderBatch, Vertex};


/// Things related to a window
pub mod window;

pub use window::{WindowEventHandler, WindowHandle, KeyCode};
use window::Window;


/// Used for timing
use std::time::{Instant};

use std::rc::Rc;
use std::cell::RefCell;


/// Starts a new game in a window
///
/// # Arguments
///
/// * 'width' - Width of the window
/// * 'height' - Height of the window
/// * 'title' - Title of the window
pub fn run_game<GameType: Game>(width: u32, height: u32, title: &str) -> Result<(), String> {
    // Create a window
    let window = match Window::new(width, height, title) {
        Err(e) => return Err(format!("{}", e)),

        Ok(window) => Rc::new(RefCell::new(window)),
    };

    // Create game
    let mut game = GameType::setup(WindowHandle::new(window.clone()));

    //////////////////
    // Run the game //
    //////////////////

    // Create a renderer
    let mut renderer = Renderer::new();
    renderer.set_clear_color((0.2, 0.2, 0.2, 1.0));

    // Measure the time each iteration of the game loop takes to complete
    let mut last_iteration_time = Instant::now();

    // Run the game loop for as long as the window and the game is open

    while game.is_running() && window.borrow().is_open() {
        // Setup OpenGL viewport
        let window_size = window.borrow().get_size(); // Rc::get_mut(&mut window).unwrap().get_size();
        if let Some((w, h)) = window_size {
            unsafe { gl::Viewport(0, 0, w as i32, h as i32) };
        }

        // Measure the time the last iteration took
        let current_iteration_time = Instant::now();
        let elapsed_time = current_iteration_time - last_iteration_time;
        let elapsed_time_secs: f32 = elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 / 1e9;
        last_iteration_time = current_iteration_time;

        // Handle all events
        let window_events = window.borrow_mut().poll_events();
        for event in window_events.into_iter() {
            window::handle_window_event(&window, event, &mut game);
        }

        // Update game
        game.update(UpdateInfo{
            dt: elapsed_time_secs
        });

        // Clear colors
        renderer.clear();

        // Render game
        game.render(&mut renderer);

        // Swap front and back buffers
        if let Err(e) = window.borrow().swap_buffers() {
            return Err(format!("{}", e));
        }
    }

    Ok(())
}
