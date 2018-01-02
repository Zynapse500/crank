

use crank;

use rand::{Rng, thread_rng};

use crank::linear::*;
use crank::{UpdateInfo, Renderer, RenderBatch};
use crank::{WindowHandle, KeyCode, MouseButton};
use crank::{View, BoundedView, CenteredView};

use super::frame_counter::FrameCounter;

pub fn run() {
    crank::run_game::<Game>(512, 512, "Crank").unwrap();
}



// A Game
struct Game {
    running: bool,
    window: WindowHandle,
    time: f32,

    batch: RenderBatch,

    frame_counter: FrameCounter,

    width: f32,
    height: f32,
    view: CenteredView,

    particles: Vec<Particle>,
    spray_cooldown: f32
}


// Handle game loop
impl crank::Game for Game {
    fn setup(window: WindowHandle) -> Game {
        Game {
            running: true,
            window,
            time: 0.0,

            batch: RenderBatch::new(),

            frame_counter: FrameCounter::new(),

            width: 0.0,
            height: 0.0,
            view: CenteredView::default(),

            particles: Vec::new(),
            spray_cooldown: 0.0
        }
    }

    fn update(&mut self, info: UpdateInfo) {
        self.time += info.dt;

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("FPS: {}   ---   Particles: {}", fps, self.particles.len()));
        }


        let mut direction = [0.0; 2];

        if self.window.key_down(KeyCode::W) { direction[1] += 1.0; }
        if self.window.key_down(KeyCode::A) { direction[0] -= 1.0; }
        if self.window.key_down(KeyCode::S) { direction[1] -= 1.0; }
        if self.window.key_down(KeyCode::D) { direction[0] += 1.0; }

        let len = vec2_length(direction);
        if len > 0.0 {
            let view_delta = vec2_scale(512.0 * info.dt, vec2_normalize(direction));
            self.view.center = vec2_add(self.view.center, view_delta);
        }

        if self.window.mouse_down(MouseButton::Left) {
            self.spray_cooldown += info.dt;

            let per_second = 10000.0;

            let mut redraw = false;

            while self.spray_cooldown > 1.0 / per_second {
                let cursor = self.window.get_cursor_position();
                let screen = self.window.window_to_ndc(cursor);
                let world = self.view.ndc_to_world(screen);

                self.particles.push(Particle::new_random(world));

                self.spray_cooldown -= 1.0 / per_second;
                redraw = true;
            }

            if redraw { self.draw();}
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.submit_batch(&self.batch);
    }
    fn is_running(&self) -> bool {
        self.running
    }
}


// Handle window events
impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        // println!("Size: {:?}", (width, height));

        self.width = width as f32;
        self.height = height as f32;

        self.view.size = [width as f32, height as f32];
        self.draw();
    }

    fn key_pressed(&mut self, key: KeyCode) {
        match key {
            KeyCode::Escape => self.running = false,

            _ => ()
        }
    }

    fn key_released(&mut self, key: KeyCode) {
        match key {

            _ => ()
        }
    }

    fn mouse_moved(&mut self, x: i32, y: i32) {
        if self.window.mouse_down(MouseButton::Right) {
            let previous_world = self.view.ndc_to_world(self.window.window_to_ndc(self.window.get_cursor_position()));
            let current_world = self.view.ndc_to_world(self.window.window_to_ndc([x, y]));

            let delta = [previous_world[0] - current_world[0], previous_world[1] - current_world[1]];
            self.view.center[0] += delta[0];
            self.view.center[1] += delta[1];
        }
    }

    fn mouse_pressed(&mut self, button: MouseButton, x: i32, y: i32) {
        let screen = self.window.window_to_ndc([x, y]);
        let world = self.view.ndc_to_world(screen);

        match button {

            _ => ()
        }
    }

    fn mouse_released(&mut self, button: MouseButton, x: i32, y: i32) {}
}


impl Game {
    fn draw(&mut self) {
        self.batch.clear();

        self.batch.set_view(self.view);

        for particle in self.particles.iter() {
            self.batch.set_fill_color(particle.color);
            self.batch.draw_rectangle(vec2_sub(particle.center, vec2_scale(0.5, particle.size)), particle.size.into());
        }
    }
}


struct Particle {
    center: [f32; 2],
    size: [f32; 2],

    color: [f32; 4]
}


impl Particle {
    pub fn new_random(position: [f32; 2]) -> Particle {
        let x = position[0];
        let y = position[1];

        let sx = thread_rng().gen_range(15.0, 100.0);
        let sy = thread_rng().gen_range(15.0, 100.0);

        let r = thread_rng().gen_range(0.2, 1.0);
        let g = thread_rng().gen_range(0.2, 1.0);
        let b = thread_rng().gen_range(0.2, 1.0);

        Particle {
            center: [x, y],
            size: [sx, sy],
            color: [r, g, b, 1.0],
        }
    }
}

