// For window and OpenGL context creation
extern crate glutin;

use glutin::GlContext;


/// Contains bindings for OpenGL
mod gl;


/// Things related to a Game
pub mod game;

pub use game::Game;

/// Things related to a window
pub mod window;

pub use window::WindowEventHandler;
use window::WindowHandle;
use window::Window;


use gl::types::*;


/// Starts a new game in a window
///
/// # Arguments
///
/// * 'width' - Width of the window
/// * 'height' - Height of the window
/// * 'title' - Title of the window
pub fn create_game<GameType: Game>(width: u32, height: u32, title: &str) -> Result<(), String> {
    let window = match Window::new(width, height, title) {
        Err(e) => return Err(format!("{}", e)),

        Ok(window) => window,
    };

    // Create game
    let mut game = GameType::setup(WindowHandle::new(&window));

    run_game(game, window);

    return Ok(());
}


// Run a game in a window
fn run_game<G: Game>(mut game: G, mut window: Window) -> Result<(), String> {

    // Set clear color
    unsafe { gl::ClearColor(0.2, 0.2, 0.2, 1.0); }

    // Run the game loop for as long as the window and the game is open
    while game.is_running() && window.is_open() {
        // Handle all events
        window.poll_events();

        // Clear colors
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        if let Err(e) = window.swap_buffers() {
            return Err(format!("{}", e));
        }
    }

    Ok(())
}

/*
use std::collections::HashSet;


// Handles window events for a game
fn handle_event<GameType: Game>(event: glutin::Event, game: &mut GameType, event_state: &mut EventHandlerState) -> EventResult {
    use glutin::{Event, WindowEvent};

    use EventResult::*;

    match event {
        Event::WindowEvent { event, .. } => {
            match event {
                WindowEvent::Closed => return CloseWindow,
                WindowEvent::KeyboardInput { input, .. } => handle_keyboard_event(input, game, event_state),

                _ => ()
            }
        }
        _ => ()
    }

    Nothing
}


// Handles a event from the keyboard
fn handle_keyboard_event<G: Game>(input: glutin::KeyboardInput, game: &mut G, event_state: &mut EventHandlerState) {
    use glutin::ElementState;

    if let Some(keycode) = input.virtual_keycode {
        match input.state {
            ElementState::Pressed => {
                if !event_state.pressed_keys.contains(&keycode) {
                    game.key_pressed(keycode);
                    event_state.pressed_keys.insert(keycode);
                }
            }
            ElementState::Released => {
                game.key_released(keycode);
                event_state.pressed_keys.remove(&keycode);
            }
        }
    }
}*/
