/// Debugging macro
#[allow(unused_macros)]
macro_rules! print_deb {
    ($var:expr) => {println!("{}: {:?}", stringify!($var), $var)};
}


macro_rules! min { ($a:expr, $b:expr) => {if $a < $b {$a} else {$b}}; }
macro_rules! max { ($a:expr, $b:expr) => {if $a > $b {$a} else {$b}}; }


// For window and OpenGL context creation
extern crate glutin;

use glutin::WindowEvent;

// For offset_of!
#[macro_use]
extern crate memoffset;

// For numeric generics
extern crate num_traits;

// For images loading
extern crate image;

/// Contains bindings for OpenGL
mod gl;

/// Things related to a Game
mod game;

pub use game::{Game, UpdateInfo};

/// Things related to an App
mod app;

pub use app::App;

/// Things related to Rendering
mod renderer;

pub use renderer::{Renderer, RenderBatch};
pub use renderer::view::*;
pub use renderer::vertex::Vertex;
pub use renderer::texture::{Texture, TextureData, TextureFilter};


/// Images
mod images;

pub use images::{Image, ImageFormat};


/// Things related to a window
mod window;

pub use window::{WindowEventHandler, WindowFileHandler, WindowHandle};
pub use window::{KeyCode, MouseButton, ScrollDelta};
use window::Window;


/// Linear transformations
pub mod linear;

pub use linear::*;


/// Shapes
mod shape;

pub use shape::*;

/// Collisions
mod collision;
pub use collision::*;

/// Physics
mod physics;
pub use physics::*;

/// Used for timing
use std::time::Instant;


/// Various std
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;


/// Starts a new game in a window
///
/// # Description
///
/// A game renders continuously, even when the user is inactive
///
/// # Arguments
///
/// * 'width' - Width of the window
/// * 'height' - Height of the window
/// * 'title' - Title of the window
pub fn run_game<GameType: Game>(width: u32, height: u32, title: &str, settings: GameSettings) -> Result<(), String> {
    let window_settings = window::WindowSettings {
        width,
        height,
        title: title.to_owned(),

        vertical_sync: settings.vertical_sync,
    };

    // Create a window
    let window = match Window::new(window_settings) {
        Err(e) => return Err(format!("{}", e)),

        Ok(window) => Rc::new(RefCell::new(window)),
    };

    // Create a renderer
    let mut renderer = Renderer::new(window.borrow().deref());
    renderer.set_clear_color(settings.clear_color);

    // Create game
    let mut game = GameType::setup(WindowHandle::new(window.clone()));


    //////////////////
    // Run the game //
    //////////////////


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
        let elapsed_time_secs: FloatType = elapsed_time.as_secs() as FloatType + elapsed_time.subsec_nanos() as FloatType / 1e9;
        last_iteration_time = current_iteration_time;

        // Handle all events
        let window_events = window.borrow_mut().poll_events();
        for event in window_events.into_iter() {
            window::handle_event(&window, event.clone(), &mut game);
        }

        // Update game
        game.update(UpdateInfo {
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


/// Settings for a game
pub struct GameSettings {
    pub vertical_sync: bool,
    pub clear_color: [f32; 4]
}


/// Starts a new app in a window.
///
/// # Description
///
/// A app only renders it's contents when a refresh is required.
/// For example when the window changes size or the user interacts with the window
///
/// # Arguments
///
/// * 'width' - Width of the window
/// * 'height' - Height of the window
/// * 'title' - Title of the window
pub fn run_app<AppType: App>(width: u32, height: u32, title: &str, settings: AppSettings) -> Result<(), String> {
    let window_settings = window::WindowSettings {
        width,
        height,
        title: title.to_owned(),

        vertical_sync: false,
    };

    // Create a window
    let window = match Window::new(window_settings) {
        Err(e) => return Err(format!("{}", e)),

        Ok(window) => Rc::new(RefCell::new(window)),
    };

    // Create an app
    let mut app = AppType::setup(WindowHandle::new(window.clone()));


    /////////////////
    // Run the app //
    /////////////////

    // Create a renderer
    let mut renderer = Renderer::new(window.borrow().deref());
    renderer.set_clear_color(settings.clear_color);


    // Run the game loop for as long as the window and the game is open
    while app.is_running() && window.borrow().is_open() {
        // Handle all events
        let window_events = window.borrow_mut().wait_events();
        for event in window_events.iter() {
            // Setup OpenGL viewport
            if let &WindowEvent::Resized(w, h) = event {
                unsafe { gl::Viewport(0, 0, w as i32, h as i32) };
            }

            let event = glutin::Event::WindowEvent {
                window_id: window.borrow().id(),
                event: event.clone(),
            };

            window::handle_event(&window, event.clone(), &mut app);
            window::handle_window_file_event(event.clone(), &mut app);
        }

        //////////////////////////////////////////
        // Re-render app when event is received //
        //////////////////////////////////////////

        if window_events.len() > 0 {
            // Clear colors
            renderer.clear();

            // Render app
            app.render(&mut renderer);

            // Swap front and back buffers
            if let Err(e) = window.borrow().swap_buffers() {
                return Err(format!("{}", e));
            }
        }
    }

    Ok(())
}


/// Settings for an app
pub struct AppSettings {
    pub clear_color: [f32; 4],
}

