
use crank;

use self::super::frame_counter::FrameCounter;

pub fn run() {
    let settings = crank::GameSettings {
        vertical_sync: false,
    };

    crank::run_game::<Game>(720, 720, "Collisions", settings).unwrap();
}


struct Game {
    running: bool,
    window: crank::WindowHandle,

    frame_counter: FrameCounter,

    batch: crank::RenderBatch,
    view: crank::CenteredView,

    rect_a: crank::Rectangle,
    rect_b: crank::Rectangle,
    line: crank::Line,

    texture_a: crank::Texture,
    texture_b: crank::Texture
}


impl Game {
    fn draw(&mut self) {
        use crank::{RenderShape, RayCast, Rectangle, Line};

        self.batch.clear();
        self.batch.set_view(self.view);


        ////////////////
        // Background //
        ////////////////

        let size = self.window.get_size();

        self.batch.set_color([0.2, 0.2, 0.2, 1.0]);
        self.batch.fill_rectangle(&Rectangle::new([0.0, 0.0], [size[0] as f32, size[1] as f32]));



        /////////////////
        // Rectangle A //
        /////////////////

        self.batch.set_color([1.0, 1.0, 1.0, 1.0]);
        self.batch.set_texture(Some(self.texture_a));

        self.batch.fill_rectangle(&self.rect_a);


        /////////////////
        // Rectangle B //
        /////////////////

        self.batch.set_texture(Some(self.texture_b));
        self.batch.fill_rectangle(&self.rect_b);


        //////////
        // Line //
        //////////

        self.batch.set_texture(None);

        self.batch.set_color([1.0, 0.0, 1.0, 1.0]);
        self.batch.draw_line(&self.line, 1.0);

        let intersections = self.rect_a.line_intersections(&self.line);
        if intersections.len() > 0 {
            let first = intersections.into_iter().min_by(|a, b| {
                a.time_of_impact.partial_cmp(&b.time_of_impact).unwrap()
            }).unwrap();

            // Draw normal
            self.batch.set_color([0.0, 1.0, 1.0, 1.0]);
            self.batch.draw_line(&Line::new(
                first.point,
                crank::vec2_add(first.point, crank::vec2_scale(25.0, first.normal))), 1.0);

            // Draw collided line
            self.batch.set_color([1.0, 1.0, 0.0, 1.0]);
            self.batch.draw_line(&Line::new(self.line.start, first.point), 1.0);

        } else {
            self.batch.set_color([1.0, 1.0, 0.0, 1.0]);
            self.batch.draw_line(&self.line, 1.0);
        }
    }


    fn mouse_to_world(&self, mouse: [i32; 2]) -> [f32; 2] {
        use crank::View;
        self.view.ndc_to_world(self.window.window_to_ndc(mouse))
    }
}


impl crank::Game for Game {
    fn setup(window: crank::WindowHandle) -> Self {
        let mut game = Game {
            running: true,
            window,

            frame_counter: FrameCounter::new(),

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView{ center: [0.0, 0.0], size: [2.0, 2.0]},

            rect_a: crank::Rectangle {center: [0.0, 0.0], size: [128.0; 2]},
            rect_b: crank::Rectangle {center: [100.0, 75.0], size: [32.0; 2]},
            line: crank::Line::new([-200.0, 100.0], [-100.0, -100.0]),

            texture_a: crank::Texture::from(crank::Image::decode(include_bytes!("res/apple.png")).unwrap()),
            texture_b: crank::Texture::from(crank::Image::decode(include_bytes!("res/banana.png")).unwrap())
        };

        game.texture_b.set_filter(crank::TextureFilter::Linear);

        game
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("FPS: {}", fps));
        }

        if self.window.mouse_down(crank::MouseButton::Right) {
            let mouse = self.mouse_to_world(self.window.get_cursor_position());

            if self.window.key_down(crank::KeyCode::LShift) {
                self.line.start = mouse;
            } else {
                self.line.end = mouse;
            }
        }

        use crank::KeyCode;

        if self.window.key_down(KeyCode::W) {
            self.rect_b.center[1] += info.dt * 100.0;
        }
        if self.window.key_down(KeyCode::S) {
            self.rect_b.center[1] -= info.dt * 100.0;
        }
        if self.window.key_down(KeyCode::D) {
            self.rect_b.center[0] += info.dt * 100.0;
        }
        if self.window.key_down(KeyCode::A) {
            self.rect_b.center[0] -= info.dt * 100.0;
        }



        // Check for overlap
        use crank::Collide;
        if let Some(overlap) = self.rect_b.overlap(&self.rect_a) {
            self.rect_b.center = crank::vec2_add(self.rect_b.center, overlap.resolve);
        }


        self.draw();
    }

    fn render(&self, renderer: &mut crank::Renderer) {
        renderer.submit_batch(&self.batch);
    }

    fn is_running(&self) -> bool {
        self.running
    }
}


impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        self.view.size = [width as f32, height as f32];
    }


    fn mouse_moved(&mut self, x: i32, y: i32) {
        if self.window.mouse_down(crank::MouseButton::Left) {
            self.rect_b.center = self.mouse_to_world([x, y]);
        }
    }


    fn key_pressed(&mut self, key: crank::KeyCode) {
        use crank::KeyCode;

        match key {
            KeyCode::Escape => self.running = false,

            _ => ()
        }
    }
}

