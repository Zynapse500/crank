use crank;

const SCALE: f64 = 1.0;


use self::super::frame_counter::FrameCounter;

pub fn run() {
    let settings = crank::GameSettings {
        vertical_sync: true,
        clear_color: [0.2; 4],
    };

    crank::run_game::<Game>(720, 720, "Collisions", settings).unwrap();
}


struct Game {
    running: bool,
    window: crank::WindowHandle,

    frame_counter: FrameCounter,

    batch: crank::RenderBatch,
    view: crank::Rectangle,

    rect_a: crank::Rectangle,
    rect_b: crank::Rectangle,
    line: crank::Line,

    sweep_start: crank::Vector2,

    accumulated_time: f64,
}


impl Game {
    fn tick(&mut self, dt: f64) {
        if self.window.mouse_down(crank::MouseButton::Right) {
            let mouse = self.mouse_to_world(self.window.get_cursor_position());

            if self.window.key_down(crank::KeyCode::LShift) {
                self.line.start = mouse;
            } else {
                self.line.end = mouse;
            }
        }

        if self.window.mouse_down(crank::MouseButton::Middle) {
            let mouse = self.mouse_to_world(self.window.get_cursor_position());

            self.sweep_start = mouse;
        }


        use crank::KeyCode;

        if self.window.key_down(KeyCode::W) {
            self.rect_b.translate([0.0, dt * 4.0].into());
        }
        if self.window.key_down(KeyCode::S) {
            self.rect_b.translate([0.0, dt * -4.0].into());
        }
        if self.window.key_down(KeyCode::D) {
            self.rect_b.translate([dt * 4.0, 0.0 ].into());
        }
        if self.window.key_down(KeyCode::A) {
            self.rect_b.translate([dt * -4.0, 0.0 ].into());
        }
        
        // Check for overlap
        use crank::Collide;
        if let Some(overlap) = self.rect_b.overlap(&self.rect_a) {
            println!("Overlap: {:?}", overlap);
            self.rect_b.translate(overlap.resolve);
        }
    }

    fn draw(&mut self) {
        use crank::{RenderShape, RayCast, Line};

        self.batch.clear();
        self.batch.set_view(self.view);

        let line_width = 1.0 / SCALE;

        /////////////////
        // Rectangle A //
        /////////////////

        self.batch.set_color([0.0, 0.0, 1.0, 1.0]);
        self.batch.draw_rectangle(&self.rect_a, line_width);


        /////////////////
        // Rectangle B //
        /////////////////

        self.batch.set_color([0.0, 1.0, 0.0, 1.0]);
        self.batch.draw_rectangle(&self.rect_b, line_width);


        //////////
        // Line //
        //////////

        self.batch.set_color([1.0, 0.0, 1.0, 1.0]);
        self.batch.draw_line(&self.line, line_width);

        if let Some(first) = self.rect_a.line_intersection(&self.line) {
            // Draw normal
            self.batch.set_color([0.0, 1.0, 1.0, 1.0]);
            self.batch.draw_line(&Line::new(
                first.point,
                first.point + 25.0 * line_width * first.normal), line_width);

            // Draw collided line
            self.batch.set_color([1.0, 1.0, 0.0, 1.0]);
            self.batch.draw_line(&Line::new(self.line.start, first.point), line_width);
        } else {
            self.batch.set_color([1.0, 1.0, 0.0, 1.0]);
            self.batch.draw_line(&self.line, line_width);
        }
    }


    fn mouse_to_world(&self, mouse: crank::Vector2i) -> crank::Vector2 {
        use crank::View;
        self.view.ndc_to_world(self.window.window_to_ndc(mouse))
    }
}


impl crank::Game for Game {
    fn setup(window: crank::WindowHandle) -> Self {
        Game {
            running: true,
            window,

            frame_counter: FrameCounter::new(),

            batch: crank::RenderBatch::new(),
            view: crank::Rectangle::centered(crank::Vector2::new(0.0, 0.0), crank::Vector2::new(2.0, 2.0)),

            rect_b: crank::Rectangle::centered(crank::Vector2::new(200.0, 250.0), crank::Vector2::new(25.0, 50.0)),

            rect_a: crank::Rectangle::centered(crank::Vector2::new(0.0, 0.0), crank::Vector2::new(200.0, 200.0)),
            line: crank::Line::new([100.0, 200.0].into(),
                                   [100.0, 0.0].into()),
            sweep_start: [150.0; 2].into(),
            accumulated_time: 0.0,
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        const UPDATE_INTERVAL: f64 = 1.0 / 2400.0;

        self.accumulated_time += info.dt;
        while self.accumulated_time > UPDATE_INTERVAL {
            self.tick(UPDATE_INTERVAL);
            self.accumulated_time -= UPDATE_INTERVAL;
        }

        self.draw();

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("FPS: {}", fps));
        }
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
        self.view = crank::Rectangle::centered(self.view.center(), [width as f64 / SCALE, height as f64 / SCALE].into());
    }


    fn mouse_moved(&mut self, x: i64, y: i64) {
        if self.window.mouse_down(crank::MouseButton::Left) {
            self.rect_b = crank::Rectangle::centered(self.mouse_to_world([x, y].into()), self.rect_b.size());
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

