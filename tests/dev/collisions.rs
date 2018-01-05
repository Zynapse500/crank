use crank;

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
    view: crank::CenteredView,

    rect_a: crank::Rectangle,
    rect_b: crank::Rectangle,
    line: crank::Line,

    sweep_start: [f32; 2],

    accumulated_time: f32,
}


impl Game {
    fn tick(&mut self, dt: f32) {
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
            self.rect_b.center[1] += dt * 256.0;
        }
        if self.window.key_down(KeyCode::S) {
            self.rect_b.center[1] -= dt * 256.0;
        }
        if self.window.key_down(KeyCode::D) {
            self.rect_b.center[0] += dt * 256.0;
        }
        if self.window.key_down(KeyCode::A) {
            self.rect_b.center[0] -= dt * 256.0;
        }

        // Check for overlap
        use crank::Collide;
        if let Some(overlap) = self.rect_b.overlap(&self.rect_a) {
            self.rect_b.center = crank::vec2_add(self.rect_b.center, overlap.resolve);
        }
    }

    fn draw(&mut self) {
        use crank::{RenderShape, RayCast, Rectangle, Line};

        self.batch.clear();
        self.batch.set_view(self.view);


        /////////////////
        // Rectangle A //
        /////////////////

        self.batch.set_color([0.0, 0.0, 1.0, 1.0]);
        self.batch.draw_rectangle(&self.rect_a, 1.0);


        /////////////////
        // Rectangle B //
        /////////////////

        self.batch.set_color([0.0, 1.0, 0.0, 1.0]);
        self.batch.draw_rectangle(&self.rect_b, 1.0);


        ///////////
        // Sweep //
        ///////////

        let mut sweep_line = Line::new(self.sweep_start, self.rect_b.center);

        let mut remaining_time = 1.0;

        let mut i = 0;

        while remaining_time > 0.0 && i < 100 {
            let sum = Rectangle {
                size: crank::vec2_add(self.rect_b.size, self.rect_a.size),
                center: self.rect_a.center,
            };

            if let Some(impact) = sum.line_intersection(&sweep_line) {
                let delta = crank::vec2_sub(sweep_line.end, sweep_line.start);

                self.batch.set_color([1.0, 0.5, 1.0, 1.0]);
                self.batch.draw_extruded_rectangle(&Rectangle {
                    center: sweep_line.start,
                    size: self.rect_b.size,
                }, crank::vec2_sub(impact.point, sweep_line.start), 1.0);

                self.batch.set_color([0.0, 1.0, 1.0, 1.0]);
                self.batch.draw_line(&Line::new(
                    impact.point, crank::vec2_add(impact.point, crank::vec2_scale(25.0, impact.normal))),
                                     1.0,
                );

                sweep_line.start = impact.point;

                let time_left = 1.0 - impact.time;
                remaining_time *= time_left;

                let dot = time_left * (impact.normal[1] * delta[0] + impact.normal[0] * delta[1]);
                let new_delta = [
                    impact.normal[1] * dot,
                    impact.normal[0] * dot
                ];

                sweep_line.end = crank::vec2_add(new_delta, sweep_line.start);
            } else {
                self.batch.set_color([1.0, 0.5, 1.0, 1.0]);
                self.batch.draw_extruded_rectangle(&Rectangle {
                    center: sweep_line.start,
                    size: self.rect_b.size,
                }, crank::vec2_sub(sweep_line.end, sweep_line.start), 1.0);
            }

            i+=1;
        }


        //////////
        // Line //
        //////////

        self.batch.set_color([1.0, 0.0, 1.0, 1.0]);
        self.batch.draw_line(&self.line, 1.0);

        let intersection = self.rect_a.line_intersection(&self.line);
        if let Some(first) = intersection {
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

            frame_counter: FrameCounter::new(),

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView { center: [0.0, 0.0], size: [2.0, 2.0] },

            rect_a: crank::Rectangle { center: [0.0, 0.0], size: [128.0; 2] },
            rect_b: crank::Rectangle { center: [100.0, 75.0], size: [32.0; 2] },
            line: crank::Line::new([-200.0, 100.0], [-100.0, -100.0]),
            sweep_start: [150.0; 2],
            accumulated_time: 0.0,
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        const UPDATE_INTERVAL: f32 = 1.0 / 2400.0;

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

