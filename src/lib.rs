
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;

extern crate gfx_window_glutin;
extern crate glutin;

extern crate time;
extern crate cgmath;

// pub mod linear;

pub mod window;
pub mod renderer;

#[path = "test_app/app.rs"]
pub mod app;


#[cfg(test)]
mod tests {

    use window;
    use app;

    #[test]
    fn hello_triangle() {
        println!("##### hello_triangle #####");

        let mut settings = window::WindowSettings::default();
        settings.title = "Hello, Triangle!".to_owned();
        settings.vertical_sync = false;

        let mut app = app::App::new();
        window::open_window(settings, &mut app);

        println!("##### end hello_triangle #####");
    }
}
