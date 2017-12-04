
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;

extern crate gfx_window_glutin;
extern crate glutin;

extern crate time;

pub mod window;
pub mod renderer;
pub mod app;


#[cfg(test)]
mod tests {

    use window;
    use renderer;

    use app;

    use glutin;

    #[test]
    fn hello_triangle() {
        println!("##### hello_triangle #####");

        let mut settings = window::WindowSettings::default();
        settings.title = "Hello, Triangle!".to_owned();

        let mut app = app::App::new();
        window::open_window(settings, &mut app);

        println!("##### end hello_triangle #####");
    }
}
