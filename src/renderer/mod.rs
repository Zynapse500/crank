
use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_device_gl;
use gfx_device_gl::Factory;

use glutin;
use gfx_window_glutin;

use window;

pub mod frame;
use self::frame::RenderFrame;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<window::ColorFormat> = "Target0",
    }
}



type R = gfx_device_gl::Resources;
type C = gfx_device_gl::CommandBuffer;

pub struct Renderer {
    device: gfx_device_gl::Device,
    factory: Factory,
    color_view: gfx::handle::RenderTargetView<R, window::ColorFormat>,
    depth_view: gfx::handle::DepthStencilView<R, window::DepthFormat>,

    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    encoder: gfx::Encoder<R, C>
}

impl Renderer {
    pub fn new(device: gfx_device_gl::Device,
               mut factory: Factory,
               color_view: gfx::handle::RenderTargetView<R, window::ColorFormat>,
               depth_view: gfx::handle::DepthStencilView<R, window::DepthFormat>) -> Self {

        let pso = Renderer::create_pipeline(&mut factory);

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        Renderer {
            device,
            factory,
            color_view,
            depth_view,

            pso,
            encoder
        }
    }


    /// Creates a new pipeline object
    fn create_pipeline(factory: &mut Factory) -> gfx::pso::PipelineState<R, pipe::Meta> {
        let set = factory.create_shader_set(
            include_bytes!("shaders/shader.vert"),
            include_bytes!("shaders/shader.frag")
        ).unwrap();

        factory.create_pipeline_state(
            &set,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer{
                samples: Some(gfx::state::MultiSample{}),
                ..gfx::state::Rasterizer::new_fill()
            },
            pipe::new()
        ).unwrap()
    }


    /// Create a new frame to render to
    pub fn get_new_frame(&mut self) -> RenderFrame {
        RenderFrame::new()
    }

    /// Renders a frame
    pub fn draw(&mut self, mut frame: RenderFrame) {
        let mut vertices = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for shape in frame.shapes.into_iter() {
            // No supplied indices
            let index_start = vertices.len() as u32;
            let index_range = (index_start..index_start + shape.len() as u32);
            indices.extend(index_range.into_iter());
            vertices.extend(shape.into_iter());
        }

        //Identity Matrix
        const TRANSFORM: Transform = Transform {
            transform:
                [[1.0, 0.0, 0.0, 0.0],
                 [0.0, 1.0, 0.0, 0.0],
                 [0.0, 0.0, 1.0, 0.0],
                 [0.0, 0.0, 0.0, 1.0]]
        };

        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&vertices, indices.as_slice());
        let transform_buffer = self.factory.create_constant_buffer(1);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            transform: transform_buffer,
            out: self.color_view.clone(),
        };

        self.encoder.clear(&self.color_view, [0.0, 0.0, 0.0, 0.0]); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)
        self.encoder.update_buffer(&data.transform, &[TRANSFORM], 0).unwrap(); //update buffers
        self.encoder.draw(&slice, &self.pso, &data); // draw commands with buffer data and attached pso
        self.encoder.flush(&mut self.device); // execute draw commands
    }


    /// Clears all leftover rendering data
    pub fn clean(&mut self) {
        self.device.cleanup();
    }


    /// Sets the viewport
    pub fn set_viewport(&mut self, window: &glutin::GlWindow, width: u32, height: u32) {
        gfx_window_glutin::update_views(window, &mut self.color_view, &mut self.depth_view);
    }
}

