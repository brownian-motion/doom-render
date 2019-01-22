#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::dpi::LogicalSize;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

const WINDOW_SIZE: LogicalSize = LogicalSize { width: 800.0, height: 800.0 };

const SQUARE: [Vertex; 4] = [
    Vertex { pos: [0.5, -0.5], color: RED },
    Vertex { pos: [-0.5, -0.5], color: GREEN },
    Vertex { pos: [-0.5, 0.5], color: BLUE },
    Vertex { pos: [0.5, 0.5], color: WHITE},
];
const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];


pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("DOOM")
        .with_dimensions(WINDOW_SIZE);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop)
            .expect("Could not create window.");

//    originally a command buffer, converted into an "Encoder" which wraps that in a backend-indpt way
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/rect_150.glslv")),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/rect_150.glslf")),
        pipe::new(),
    ).expect("Could not establish pipeline.");
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, INDICES);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    let mut running = true;
    while running { // looping on mut bool so that we could break from inside a lambda by changing it, and we clean up reliably
        events_loop.poll_events(|event: glutin::Event| {
            match event {
                glutin::Event::WindowEvent { window_id: _, event } => {
                    use glutin::WindowEvent::*;
                    match event {
                        KeyboardInput { device_id: _, input: glutin::KeyboardInput { scancode: _, state: _, virtual_keycode: Some(glutin::VirtualKeyCode::Escape), modifiers: _ } }
                        | CloseRequested =>
                            running = false,
                        Resized(_) => {
                            gfx_glutin::update_views(&window, &mut data.out, &mut main_depth);
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        });

        encoder.clear(&data.out, BLACK); // draw the background
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device); // sends the commands in the buffer to the GPU, so this loop is completely executed in the same frame
        window.swap_buffers().expect("Could apply the current buffer to the window. Aborting.");
        device.cleanup();
    }
}

// defines variables and such in the target GPU language
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}