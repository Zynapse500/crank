
use ::{WindowHandle, WindowEventHandler, WindowFileHandler};
use ::Renderer;

pub trait App: WindowEventHandler + WindowFileHandler {
    /// Creates a new app.
    /// Also provides a handle to the window the app is contained within.
    ///
    /// # Arguments
    ///
    /// * 'window' - Handle that can modify the window
    fn setup(window: WindowHandle) -> Self;


    /// Render the contents of the game to a renderer.
    /// Only called when a refresh is necessary.
    fn render(&self, renderer: &mut Renderer);


    /// Determines if the game is running or not
    fn is_running(&self) -> bool { true }
}

