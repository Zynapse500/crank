use std::collections::HashSet;

use gfx;
use gfx_window_glutin as gfx_glutin;

use glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

use time;

use renderer::Renderer;
use renderer::frame::RenderFrame;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


pub use glutin::VirtualKeyCode as KeyCode;
pub use glutin::MouseButton as MouseButton;


pub trait WindowHandler {

    /// Main Loop
    fn init(&mut self);
    fn update(&mut self, delta_time: f32);

    fn render(&mut self, frame: &mut RenderFrame);

    /// Events
    fn resized(&mut self, width: u32, height: u32) {}
    fn key_pressed(&mut self, key: KeyCode) {}
    fn key_released(&mut self, key: KeyCode) {}
    fn mouse_pressed(&mut self, button: MouseButton) {}
    fn mouse_released(&mut self, button: MouseButton) {}

    fn closed(&mut self) {}

    /// State
    fn is_running(&self) -> bool;
}


pub struct WindowSettings {
    pub width: u32,
    pub height: u32,

    pub title: String,

    pub vertical_sync: bool,
    pub samples: u16,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            width: 512,
            height: 512,

            title: "Locus".to_owned(),

            vertical_sync: true,
            samples: 8
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
        .with_multisampling(settings.samples)
        .with_vsync(settings.vertical_sync);


    let (window, device, factory, color_view, depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

    // Create renderer
    let mut renderer= Renderer::new(device, factory, color_view, depth_view);


    // Setup handler
    window_handler.init();


    // The time point of the last frame
    let mut previous_frame_time = time::precise_time_ns();

    // State of events
    let mut event_state = EventState::new();

    // Main loop
    'main:loop {
        if !window_handler.is_running() { break 'main; }

        // Handle events
        let events = get_events(&mut events_loop);
        for event in events {
            if let Some(action) = handle_event(window_handler, event, &mut event_state) {
                match action {
                    EventAction::Close => {
                        window_handler.closed();
                        break 'main;
                    },

                    // Resize viewport to fit window
                    EventAction::Rezise(w, h) => {
                        renderer.set_viewport(&window,w, h);
                    }
                }
            }
        }


        // Calculate how long it took to render previous frame
        let current_time = time::precise_time_ns();
        let delta_time = (current_time - previous_frame_time) as f32 / 1e9;
        previous_frame_time = current_time;

        window_handler.update(delta_time);

        // Get the next frame and render to it
        let mut frame = renderer.get_new_frame();
        window_handler.render(&mut frame);
        renderer.draw(frame);

        window.swap_buffers().unwrap();

        renderer.clean();
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
    Close,
    Rezise(u32, u32)
}

struct EventState {
    /// Filter out repeated key presses
    pub pressed_keys: HashSet<KeyCode>
}

impl EventState {
    pub fn new() -> Self {
        EventState {
            pressed_keys: HashSet::new(),
        }
    }
}


fn handle_event<W: WindowHandler>(window_handler: &mut W, event: glutin::Event, event_state: &mut EventState) -> Option<EventAction> {
    match event {
        glutin::Event::WindowEvent {event, ..} => {
            match event {
                // Keyboard input
                glutin::WindowEvent::KeyboardInput { input: glutin::KeyboardInput {
                        virtual_keycode: Some(key), state, ..
                    }, .. } =>
                {
                    match state {
                        glutin::ElementState::Pressed => {
                            if !event_state.pressed_keys.contains(&key) {
                                window_handler.key_pressed(key);
                                event_state.pressed_keys.insert(key);
                            }
                        },
                        glutin::ElementState::Released => {
                            window_handler.key_released(key);
                            event_state.pressed_keys.remove(&key);
                        },
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
                    return Some(EventAction::Rezise(width, height));
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

