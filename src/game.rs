


use ::window::WindowHandle;
use ::window::WindowEventHandler;

/// What is a game?
/// - Contains callback functions for rendering and updates to the window
/// - Handles event callbacks

pub trait Game: WindowEventHandler {
    /// Creates a new game.
    /// Provides a handle to the window the game is contained within.
    fn setup(window: WindowHandle);


    // TODO: Implement 'UpdateInfo'

    // /// Updates the contents of the game
    // fn update(info: UpdateInfo);


    // TODO: Implement 'Renderer'

    // /// Render the contents of the game to a renderer
    // fn render(renderer: &mut Renderer);
}
