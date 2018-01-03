use crank;


const TEXTURE_SCALE: f32 = 4.0;


use super::frame_counter::FrameCounter;

pub fn run() {
    let settings = crank::GameSettings {
        vertical_sync: false,
    };

    crank::run_game::<Game>(900, 900, "Textures", settings).unwrap();
}


struct Game {
    running: bool,
    window: crank::WindowHandle,

    frame_counter: FrameCounter,
    time: f32,

    batch: crank::RenderBatch,
    view: crank::CenteredView,

    texture: crank::Texture,
    texture_size: [f32; 2],
    texture_offset: [f32; 2],

    color_filter: [f32; 4],
}


impl Game {
    fn draw(&mut self) {
        use crank::{RenderShape, Rectangle};

        self.batch.clear();
        self.update_view();

        self.batch.set_texture(Some(self.texture));
        // self.batch.set_fill_color(self.color_filter);
        self.batch.fill_rectangle(&Rectangle::new([0.0, 0.0], self.texture_size));
    }

    fn update_view(&mut self) {
        self.batch.set_view(self.view)
    }
}


impl crank::Game for Game {
    fn setup(window: crank::WindowHandle) -> Self {
        let image = crank::Image::decode(include_bytes!("res/banana.png")).unwrap();

        let mut texture = crank::Texture::from(image.clone());
        texture.set_filter(crank::TextureFilter::Nearest);

        Game {
            running: true,
            window,

            frame_counter: FrameCounter::new(),
            time: 0.0,

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView::default(),

            texture,
            texture_size: crank::vec2_scale(TEXTURE_SCALE, [image.get_width() as f32, image.get_height() as f32]),
            texture_offset: [0.0, 0.0],

            color_filter: [1.0; 4],
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        self.time += info.dt;

        self.color_filter[0] = self.time.cos() / 2.0 + 0.5;
        self.color_filter[1] = self.time.sin() / 2.0 + 0.5;
        self.color_filter[2] = (self.time / 10.0f32).sin() / 2.0 + 0.5;

        self.texture_offset[1] = self.time.sin() * 20.0;

        self.draw();

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("Textures   ---   FPS: {}", fps));
        }
    }

    fn render(&self, renderer: &mut crank::Renderer) {
        renderer.set_clear_color([0.5; 4]);
        renderer.submit_batch(&self.batch);
    }

    fn is_running(&self) -> bool {
        self.running
    }
}


impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        self.view.center = [0.0, 0.0];
        self.view.size = [width as f32, height as f32];

        self.update_view();
    }


    fn key_pressed(&mut self, key: crank::KeyCode) {
        match key {
            crank::KeyCode::Escape => self.running = false,

            _ => ()
        }
    }
}
