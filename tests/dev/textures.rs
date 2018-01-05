#[allow(unused_macros)]
macro_rules! print_deb {
  ($var:expr) => {println!("{}: {:?}", stringify!($var), $var)};
}


use crank;



const TEXTURE_SCALE: f32 = 4.0;


use super::frame_counter::FrameCounter;

pub fn run() {
    let settings = crank::GameSettings {
        vertical_sync: false,
        clear_color: [0.3; 4],
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

    apple_texture: crank::Texture,
    banana_texture: crank::Texture,
    chili_texture: crank::Texture,

    texture_size: [f32; 2],
    texture_offset: [f32; 2],

    chili_offset: [f32; 2],
}


impl Game {
    fn draw(&mut self) {
        use crank::{RenderShape, Rectangle};

        self.batch.clear();
        self.update_view();


        self.batch.set_texture(Some(self.banana_texture));
        self.batch.fill_rectangle(&Rectangle::new([80.0, 0.0], self.texture_size));

        self.batch.set_texture(Some(self.chili_texture));
        self.batch.fill_rectangle(&Rectangle::new(self.chili_offset, self.texture_size));

        self.batch.set_texture(Some(self.apple_texture));
        self.batch.fill_rectangle(&Rectangle::new(self.texture_offset, self.texture_size));

        self.batch.set_texture(Some(self.banana_texture));
        self.batch.fill_rectangle(&Rectangle::new([-80.0, 0.0], self.texture_size));

        self.batch.set_texture(Some(self.apple_texture));
        self.batch.fill_rectangle(&Rectangle::new([-40.0, 40.0], self.texture_size));
    }

    fn move_chili(&mut self, dt: f32) {
        use crank::KeyCode;

        let mut direction = [0.0; 2];

        if self.window.key_down(KeyCode::D) { direction[0] += 1.0; }
        if self.window.key_down(KeyCode::A) { direction[0] -= 1.0; }
        if self.window.key_down(KeyCode::W) { direction[1] += 1.0; }
        if self.window.key_down(KeyCode::S) { direction[1] -= 1.0; }

        if direction[0] != 0.0 || direction[1] != 0.0 {
            let delta = crank::vec2_scale(256.0 * dt, crank::vec2_normalize(direction));
            self.chili_offset = crank::vec2_add(self.chili_offset, delta);
        }
    }

    fn update_view(&mut self) {
        self.batch.set_view(self.view)
    }
}


impl crank::Game for Game {
    fn setup(window: crank::WindowHandle) -> Self {
        use crank::{Texture, Image};
        let image = crank::Image::decode(include_bytes!("res/apple.png")).unwrap();

        let texture = crank::Texture::from(image.clone());

        Game {
            running: true,
            window,

            frame_counter: FrameCounter::new(),
            time: 0.0,

            batch: crank::RenderBatch::new(),
            view: crank::CenteredView::default(),

            apple_texture: texture,
            chili_texture: Texture::from(Image::decode(include_bytes!("res/chili.png")).unwrap()),
            banana_texture: Texture::from(Image::decode(include_bytes!("res/banana.png")).unwrap()),

            texture_size: crank::vec2_scale(TEXTURE_SCALE, [image.get_width() as f32, image.get_height() as f32]),
            texture_offset: [0.0, 0.0],

            chili_offset: [40.0, 40.0]
        }
    }

    fn update(&mut self, info: crank::UpdateInfo) {
        self.time += info.dt;
        self.texture_offset[1] = self.time.sin() * 40.0;
        self.move_chili(info.dt);

        self.draw();

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("Textures   ---   FPS: {}", fps));
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
