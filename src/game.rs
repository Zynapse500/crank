


use ::{WindowHandle, WindowEventHandler};
use ::Renderer;

use ::FloatType;

/// What is a game?
/// - Contains callback functions for handling rendering and updates to the window
/// - Handles event callbacks

pub trait Game: WindowEventHandler {
    /// Creates a new game.
    /// Also provides a handle to the window the game is contained within.
    ///
    /// # Arguments
    ///
    /// * 'window' - Handle that can modify the window
    fn setup(window: WindowHandle) -> Self;


    /// Updates the contents of the game
    fn update(&mut self, info: UpdateInfo);


    /// Render the contents of the game to a renderer
    fn render(&self, renderer: &mut Renderer);



    /// Determines if the game is running or not
    fn is_running(&self) -> bool { true }
}


pub struct UpdateInfo {
    // Change in time, in seconds
    pub dt: FloatType
}

