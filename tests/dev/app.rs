
use crank;

use std::path::Path;
use std::env;

pub fn run() {
    crank::run_app::<App>(680, 680,"Image Viewer").unwrap();
}


struct App {
    running: bool,
    window: crank::WindowHandle,

    batch: crank::RenderBatch,

    zoom: f32,

    texture: Option<crank::Texture>,
    image_size: (f32, f32)
}


impl App {
    fn draw(&mut self) {
        self.batch.clear();

        let view = self.calculate_view();
        self.batch.set_view(view);

        let (w, h) = self.image_size;

        self.batch.set_texture(self.texture);
        self.batch.set_fill_color([1.0; 4]);
        self.batch.draw_rectangle([-w / 2.0, -h / 2.0], [w, h]);
    }


    fn calculate_view(&self) -> crank::CenteredView {
        let w = self.window.get_width() as f32 / self.zoom;
        let h = self.window.get_height() as f32 / self.zoom;

        crank::CenteredView {
            center: [0.0; 2],
            size: [w, h]
        }
    }

    fn create_texture(image: Option<crank::Image>) -> Option<crank::Texture> {
        if let Some(image) = image {
            let mut texture = crank::Texture::from(image);
            texture.set_min_mag_filter(crank::TextureFilter::Linear,
                                       crank::TextureFilter::Nearest);

            Some(texture)
        } else {
            None
        }
    }
}


impl crank::App for App {
    fn setup(window: crank::WindowHandle) -> Self {
        let args: Vec<String> = env::args().collect();
        let image = if args.len() > 1 {
            println!("Args: {:?}", args);

            match crank::Image::load_png(Path::new(&args[1])) {
                Ok(image) => Some(image),
                Err(e) => panic!(e)
            }
        } else {
            None
        };

        let size = match image {
            Some(image) => [image.get_width(), image.get_height()],
            None => [100.0, 100.0]
        };

        App {
            running: true,
            window,

            batch: crank::RenderBatch::new(),

            zoom: zoom_from_sizes(window.get_size(), size),

            texture: App::create_texture(image),
            image_size: (0.0, 0.0)
        }
    }

    fn render(&self, renderer: &mut crank::Renderer) {
        renderer.set_clear_color([0.2, 0.2, 0.2, 1.0]);
        renderer.submit_batch(&self.batch);
    }

    fn is_running(&self) -> bool {
        self.running
    }
}


impl crank::WindowEventHandler for App {
    fn size_changed(&mut self, width: u32, height: u32) {
        self.draw();
    }


    fn key_pressed(&mut self, key: crank::KeyCode) {
        match key {
            crank::KeyCode::Escape => self.running = false,

            _ => ()
        }
    }


    fn mouse_scrolled(&mut self, delta: crank::ScrollDelta) {
        match delta {
            crank::ScrollDelta::LineDelta(x, y) => {

                const ZOOM_AMOUNT: f32 = 1.5;

                if y > 0.0 {
                    self.zoom *= ZOOM_AMOUNT;
                } else {
                    self.zoom /= ZOOM_AMOUNT;
                }

                if self.zoom > 64.0 {
                    self.zoom = 64.0;
                }
                if self.zoom < 0.1 {
                    self.zoom = 0.1;
                }

                self.draw();
            }

            crank::ScrollDelta::PixelDelta(x, y) => {}
        }
    }
}


impl crank::WindowFileHandler for App {
    fn file_dropped(&mut self, path: &Path) {
        println!("Dropped file: {:?}", path.to_str());

        let image = crank::Image::load_png(path);

        match image {
            Err(e) => println!("{}", e),

            Ok(image) => {
                let (w, h) = image.get_size();
                self.image_size = (w as f32, h as f32);

                self.texture = App::create_texture(Some(image));

                self.zoom = zoom_from_sizes( self.window.get_size(), [w, h]);
                self.draw();
            }
        }
    }
}


fn zoom_from_sizes(bound: [u32; 2], rect: [u32; 2]) -> f32 {
    let px = bound[0] as f32 / rect[0] as f32;
    let py = bound[1] as f32 / rect[1] as f32;

    let zoom_x = 1.0 / px;
    let zoom_y = 1.0 / py;

    println!("Zoom x/y: ({}, {})", zoom_x, zoom_y);

    if zoom_x > zoom_y {zoom_x} else {zoom_y}
}


