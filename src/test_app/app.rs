
use window;
use window::{KeyCode, RenderFrame};

use renderer::camera::OrthographicCamera;

pub struct App {
    window: WindowState,

    colors: Vec<[f32; 4]>,
    rotate_timer: Timer,

    frame_rate_counter: FrameCounter,

    camera: OrthographicCamera,
}

impl App {
    pub fn new() -> Self {
        App {
            window: WindowState::default(),

            colors: vec![[1.0, 0.1, 0.1, 1.0], [0.1, 1.0, 0.1, 1.0], [0.1, 0.1, 1.0, 1.0]],

            rotate_timer: Timer::new(0.2, true),
            frame_rate_counter: FrameCounter::new(0.5),

            camera: OrthographicCamera::default(),
        }
    }
}


impl window::WindowHandler for App {

    //////////////////////////////////////
    // Main loop /////////////////////////
    //////////////////////////////////////

    fn init(&mut self) {
        println!("Created new App");
    }

    fn update(&mut self, dt: f32) {
        if let Some(fps) = self.frame_rate_counter.update(dt) {
            println!("FPS: {}", fps.round());
        }

        if let Some(_) = self.rotate_timer.update(dt) {
            self.rotate_colors();
        }
    }

    fn render(&mut self, frame: &mut RenderFrame) {
        frame.set_camera(self.camera.clone());

        let w = self.window.width as f32;
        let h = self.window.height as f32;

        frame.set_color(&[0.2, 0.5, 0.7, 1.0]);
        const MARGIN: f32 = 8.0;
        frame.draw_rect(MARGIN, MARGIN, w - 2.0 * MARGIN, h - 2.0 * MARGIN);

        frame.set_color(&self.colors[0]);
        frame.draw_polygon(&[
            [0.5 * w, 0.25 * h],
            [0.75 * w, 0.75 * h],
            [0.25 * w, 0.75 * h],
        ]);


        frame.set_color(&self.colors[1]);
        frame.draw_polygon(&[
            [0.5 * w, 0.75 * h],
            [0.75 * w, 0.25 * h],
            [0.25 * w, 0.25 * h],
        ]);

        frame.set_color(&self.colors[2]);
        frame.draw_polygon(&[
            [0.5 * w, 0.25 * h],
            [0.625 * w, 0.5 * h],
            [0.5 * w, 0.75 * h],
            [0.375 * w, 0.5 * h],
        ]);
    }

    //////////////////////////////////////
    // Events ////////////////////////////
    //////////////////////////////////////

    fn key_pressed(&mut self, key: KeyCode) {
        println!("Pressed: {:?}", key);


        match key {
            KeyCode::Escape => self.window.open = false,

            KeyCode::Tab => self.rotate_colors(),

            _ => ()
        }
    }


    fn resized(&mut self, width: u32, height: u32) {
        self.window.width = width;
        self.window.height = height;


        self.camera = OrthographicCamera::new(0.0, width as f32,
                                              height as f32, 0.0);
    }


    //////////////////////////////////////
    // State /////////////////////////////
    //////////////////////////////////////

    fn is_running(&self) -> bool {
        self.window.open
    }
}


impl App {
    fn rotate_colors(&mut self) {
        let first = self.colors.remove(0);
        self.colors.push(first);
    }
}








/// Trigger event when time has passed
struct Timer {
    duration: f32,
    elapsed: f32,
    repeat: bool,
}

impl Timer {
    pub fn new(duration: f32, repeat: bool) -> Self
    {
        Timer {
            duration,
            elapsed: 0.0,
            repeat
        }
    }


    /// Returns the elapsed time if duration is exceeded
    pub fn update(&mut self, delta: f32) -> Option<f32> {
        self.elapsed += delta;

        if self.elapsed >= self.duration {
            let elapsed = self.elapsed;

            if self.repeat {
                self.elapsed = 0.0;
            }

            return Some(elapsed);
        }

        return None;
    }
}


struct FrameCounter {
    timer: Timer,
    frames: u32,
}


impl FrameCounter {
    pub fn new(interval: f32) -> Self {
        FrameCounter {
            timer: Timer::new(interval, true),
            frames: 0
        }
    }


    pub fn update(&mut self, dt: f32) -> Option<f32> {
        self.frames += 1;

        if let Some(elapsed) = self.timer.update(dt) {
            let fps = self.frames as f32 / elapsed;
            self.frames = 0;

            return Some(fps);
        }

        None
    }
}





struct WindowState {
    pub open: bool,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            open: true,
            width: 0,
            height: 0,
        }
    }
}
