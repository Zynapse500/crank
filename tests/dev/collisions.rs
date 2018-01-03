
use crank;

pub fn run() {
    crank::run_game::<Game>(720, 720, "Collisions").unwrap();
}


struct Game {
    running: bool,
    window: crank::WindowHandle,

    batch: crank::RenderBatch,
    view: crank::CenteredView,

    rect_a: crank::Rectangle,
    rect_b: crank::Rectangle,
    line: crank::Line
}


impl Game {
    fn draw(&mut self) {
        use crank::{RenderShape, Collide, RayCast, Rectangle, Line};

        self.batch.clear();
        self.batch.set_view(self.view);


        /////////////////
        // Rectangle A //
        /////////////////

        self.batch.set_color([0.0, 0.0, 1.0, 1.0]);

        let mouse = self.mouse_to_world(self.window.get_cursor_position());
        if self.rect_a.contains(mouse) {
            self.batch.fill_rectangle(&self.rect_a);
        } else {
            self.batch.draw_rectangle(&self.rect_a, 1.0);
        }


        /////////////////
        // Rectangle B //
        /////////////////

        // Check for intersections
        if self.rect_b.intersects(&self.rect_a) {
            self.batch.set_color([1.0, 0.0, 0.0, 1.0]);
        } else {
            self.batch.set_color([0.0, 1.0, 0.0, 1.0]);
        }

        self.batch.draw_rectangle(&self.rect_b, 1.0);

        // Check for overlap
        if let Some(overlap) = self.rect_b.overlap(&self.rect_a) {
            let rect_c = Rectangle::new(
                crank::vec2_add(self.rect_b.center, overlap.resolve),
                self.rect_b.size
            );

            self.batch.set_color([1.0, 1.0, 1.0, 1.0]);
            self.batch.draw_rectangle(&rect_c, 1.0);

            self.batch.draw_line(&Line::new(rect_c.center, self.rect_b.center), 1.0);
        }


        //////////
        // Line //
        //////////

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
        Game {
            running: true,
            window,

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView{ center: [0.0, 0.0], size: [2.0, 2.0]},

            rect_a: crank::Rectangle {center: [0.0, 0.0], size: [100.0, 50.0]},
            rect_b: crank::Rectangle {center: [100.0, 75.0], size: [25.0, 150.0]},
            line: crank::Line::new([-200.0, 100.0], [-100.0, -100.0])
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        self.draw();

        if self.window.mouse_down(crank::MouseButton::Right) {
            let mouse = self.mouse_to_world(self.window.get_cursor_position());

            if self.window.key_down(crank::KeyCode::LShift) {
                self.line.start = mouse;
            } else {
                self.line.end = mouse;
            }
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

