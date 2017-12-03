use gfx;
// use gfx::traits::FactoryExt;
use gfx::Device;

use gfx_window_glutin as gfx_glutin;

use glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

use time;


pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub trait WindowHandler {

    /// Main Loop
    fn init(&mut self);
    fn update(&mut self, delta_time: f32);
    // fn render(&mut self, renderer: Renderer);

    /// Events
    fn resized(&mut self, width: u32, height: u32) {}
    fn key_pressed(&mut self, key: glutin::VirtualKeyCode) {}
    fn key_released(&mut self, key: glutin::VirtualKeyCode) {}
    fn mouse_pressed(&mut self, button: glutin::MouseButton) {}
    fn mouse_released(&mut self, button: glutin::MouseButton) {}

    fn closed(&mut self) {}

    /// State
    fn is_running(&self) -> bool;
}



pub struct WindowSettings {
    pub width: u32,
    pub height: u32,

    pub title: String,

    pub vertical_sync: bool
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            width: 512,
            height: 512,

            title: "Locus".to_owned(),

            vertical_sync: true
        }
    }
}




pub fn open_window<W: WindowHandler>(settings: WindowSettings, window_handler: &mut W) {
    let mut events_loop = glutin::EventsLoop::new();

    let window_builder = glutin::WindowBuilder::new()
        .with_title(settings.title)
        .with_dimensions(settings.width, settings.height);

    let context_builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(settings.vertical_sync);

    let (window, mut device, mut factory, color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);


    // Setup handler
    window_handler.init();


    // The time point of the last frame
    let mut previous_frame_time = time::precise_time_ns();

    // Main loop
    'main:loop {
        if !window_handler.is_running() { break 'main; }

        // Handle events
        let events = get_events(&mut events_loop);
        for event in events {
            if let Some(action) = handle_event(window_handler, event) {
                match action {
                    EventAction::Close => {
                        window_handler.closed();
                        break 'main;
                    }
                }
            }
        }


        // Calculate how long it took to render previous frame
        let current_time = time::precise_time_ns();
        let delta_time = (current_time - previous_frame_time) as f32 / 1e9;
        previous_frame_time = current_time;

        window_handler.update(delta_time);
        // window_handler.render(renderer);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}


fn get_events(events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
    let mut events = Vec::new();

    events_loop.poll_events(|event| {
        events.push(event);
    });

    events
}


enum EventAction {
    Close
}

fn handle_event<W: WindowHandler>(window_handler: &mut W, event: glutin::Event) -> Option<EventAction> {
    match event {
        glutin::Event::WindowEvent {event, ..} => {
            match event {
                // Keyboard input
                glutin::WindowEvent::KeyboardInput { input: glutin::KeyboardInput {
                        virtual_keycode: Some(key), state, ..
                    }, .. } =>
                {
                    match state {
                        glutin::ElementState::Pressed => window_handler.key_pressed(key),
                        glutin::ElementState::Released => window_handler.key_released(key),
                    }
                }

                // Mouse input
                glutin::WindowEvent::MouseInput {button, state, ..} => {
                    match state {
                        glutin::ElementState::Pressed => window_handler.mouse_pressed(button),
                        glutin::ElementState::Released => window_handler.mouse_released(button),
                    }
                }


                // Window resize
                glutin::WindowEvent::Resized(width, height) => {
                    window_handler.resized(width, height);
                }

                // Window close
                glutin::WindowEvent::Closed => return Some(EventAction::Close),

                _ => ()
            }
        }

        _ => ()
    }

    None
}

