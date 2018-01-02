use crank;

use super::frame_counter::FrameCounter;

pub fn run() {
    crank::run_game::<Game>(900, 900, "Textures").unwrap();
}


struct Game {
    window: crank::WindowHandle,

    frame_counter: FrameCounter,
    time: f32,

    batch: crank::RenderBatch,
    view: crank::CenteredView,

    texture: crank::Texture,
    texture_size: [f32; 2],

    color_filter: [f32; 4]
}


impl Game {
    fn draw(&mut self) {
        self.batch.clear();
        self.update_view();

        self.batch.set_texture(Some(self.texture));
        self.batch.set_fill_color(self.color_filter);
        self.batch.draw_rectangle(
            [-self.texture_size[0] / 2.0, -self.texture_size[1] / 2.0],
            [self.texture_size[0], self.texture_size[1]]
        );
    }

    fn update_view(&mut self) {
        self.batch.set_view(self.view)
    }
}


impl crank::Game for Game {
    fn setup(window: crank::WindowHandle) -> Self {
        let pixels = crank::TextureData::RGBA(
            &[
                255, 255, 255, 255,     255, 0,   0,   255,     255, 255, 0,   255,
                0,   0,   0,   255,     0,   255, 255, 255,     0,   0,   255, 255,
                255, 0,   255, 255,     0,   255,   0,   255,     255, 0,   0,   255,
            ]
        );
        println!("Pixels: {:?}", pixels);

        let mut texture = crank::Texture::new(3, 3, pixels);
        texture.set_filter(crank::TextureFilter::Linear);

        Game {
            window,

            frame_counter: FrameCounter::new(),
            time: 0.0,

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView::default(),

            texture,
            texture_size: [0.0, 0.0],

            color_filter: [1.0; 4]
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        self.time += info.dt;

        self.color_filter[0] = self.time.cos() / 2.0 + 0.5;
        self.color_filter[1] = self.time.sin() / 2.0 + 0.5;
        self.color_filter[1] = (self.time / 10.0f32).sin() / 2.0 + 0.5;

        self.draw();

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("Textures   ---   FPS: {}", fps));
        }
    }

    fn render(&self, renderer: &mut crank::Renderer) {
        renderer.set_clear_color([0.0; 4]);
        renderer.submit_batch(&self.batch);
    }
}


impl crank::WindowEventHandler for Game {
    fn size_changed(&mut self, width: u32, height: u32) {
        self.view.center = [0.0, 0.0];
        self.view.size = [width as f32, height as f32];

        self.texture_size[0] = width as f32 - 0.0;
        self.texture_size[1] = height as f32 - 0.0;

        self.update_view();
    }
}
