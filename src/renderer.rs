
use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_device_gl;
use gfx_device_gl::Factory;

use window;

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

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shader.vert"),
            include_bytes!("shaders/shader.frag"),
            pipe::new()
        ).unwrap();

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

    /// Create a new frame to render to
    pub fn get_new_frame(&mut self) -> RenderFrame {
        RenderFrame {}
    }

    /// Renders a frame
    pub fn draw(&mut self, frame: RenderFrame) {
        const TRIANGLE: [Vertex; 3] = [
            Vertex { pos: [ -0.5, -0.5, 0.0, 1.0 ], color: [1.0, 0.0, 0.0, 1.0] },
            Vertex { pos: [  0.5, -0.5, 0.0, 1.0 ], color: [0.0, 1.0, 0.0, 1.0] },
            Vertex { pos: [  0.0,  0.5, 0.0, 1.0 ], color: [0.0, 0.0, 1.0, 1.0] },
        ];
        //Identity Matrix
        const TRANSFORM: Transform = Transform {
            transform:
                [[1.0, 0.0, 0.0, 0.0],
                 [0.0, 1.0, 0.0, 0.0],
                 [0.0, 0.0, 1.0, 0.0],
                 [0.0, 0.0, 0.0, 1.0]]
        };

        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
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


    pub fn clean(&mut self) {
        self.device.cleanup();
    }
}


pub struct RenderFrame {

}
