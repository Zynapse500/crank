


use ::window::WindowHandle;
use ::window::WindowEventHandler;

/// What is a game?
/// - Contains callback functions for rendering and updates to the window
/// - Handles event callbacks

pub trait Game: WindowEventHandler {
    /// Creates a new game.
    /// Also provides a handle to the window the game is contained within.
    ///
    /// # Arguments
    ///
    /// * 'window' - Handle that can modify the window
    fn setup(window: WindowHandle) -> Self;


    // TODO: Implement 'UpdateInfo'

    // /// Updates the contents of the game
    // fn update(info: UpdateInfo);


    // TODO: Implement 'Renderer'

    // /// Render the contents of the game to a renderer
    // fn render(renderer: &mut Renderer);



    /// Determines if the game is running or not
    fn is_running(&self) -> bool { true }
}
