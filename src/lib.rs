
#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

extern crate time;

pub mod window;

#[cfg(test)]
mod tests {
    use window;
    use glutin;

    #[test]
    fn hello_triangle() {
        println!("##### hello_triangle #####");


        struct App {
            running: bool,
        }

        impl App {
            pub fn new() -> Self {
                App {
                    running: true
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
                println!("Delta: {}", dt);
            }

            //////////////////////////////////////
            // Events ////////////////////////////
            //////////////////////////////////////

            fn key_pressed(&mut self, key: glutin::VirtualKeyCode) {
                println!("Pressed: {:?}", key);

                use glutin::VirtualKeyCode;
                match key {
                    VirtualKeyCode::Escape => self.running = false,
                    _ => ()
                }
            }

            fn key_released(&mut self, key: glutin::VirtualKeyCode) {
                println!("Released: {:?}", key);
            }

            fn mouse_pressed(&mut self, button: glutin::MouseButton) {
                println!("Pressed: {:?}", button);
            }

            fn mouse_released(&mut self, button: glutin::MouseButton) {
                println!("Released: {:?}", button);
            }



            fn resized(&mut self, width: u32, height: u32) {
                println!("Resized: ({}, {})", width, height);
            }

            fn closed(&mut self) {
                println!("Closed!");
            }


            //////////////////////////////////////
            // State /////////////////////////////
            //////////////////////////////////////

            fn is_running(&self) -> bool {
                self.running
            }
        }


        let mut settings = window::WindowSettings::default();
        settings.title = "Hello, Triangle!".to_owned();

        let mut app = App::new();
        window::open_window(settings, &mut app);

        println!("##### end hello_triangle #####");
    }
}
