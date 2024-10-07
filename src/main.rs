use std::{fs, time::Instant};

use glium::{implement_vertex, uniform, Surface};
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
}
implement_vertex!(Vertex, pos);

#[derive(Copy, Clone)]
struct Float3 {
    pos: [f32; 3],
}
implement_vertex!(Float3, pos);

fn draw(ui: &mut Ui, code: &mut String) {
    ui.window("code")
        .size([500.0, 500.0], Condition::FirstUseEver)
        .build(|| {
            ui.input_text_multiline("code", code, [-1.0, -1.0]).build();
        });
}

struct Float2 {
    x: f32,
    y: f32,
}

fn main() {
    let mut vertex_shader = fs::read_to_string("./vert.glsl").unwrap_or_default();
    let mut fragment_shader = fs::read_to_string("./frag.glsl").unwrap_or_default();

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let builder = WindowBuilder::new()
        .with_title("Shaderpad")
        .with_inner_size(LogicalSize::new(1920, 1080));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(builder)
        .build(&event_loop);
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let vertex1 = Vertex { pos: [-0.5, -0.5] };
    let vertex2 = Vertex { pos: [0.0, 0.5] };
    let vertex3 = Vertex { pos: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let mut i_resolution = Float3 {
        pos: [1920.0, 1080.0, 1.0],
    };

    let mut program =
        glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();
    let first_frame = Instant::now();
    let mut last_frame = Instant::now();
    let mut run = true;
    event_loop
        .run(move |event, window_target| {
            match event {
                Event::NewEvents(start_cause) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.frame();

                    let mut run = true;

                    draw(ui, &mut fragment_shader);

                    if !run {
                        window_target.exit();
                    }

                    let mut target = display.draw();
                    target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);

                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    match glium::Program::from_source(
                        &display,
                        &vertex_shader,
                        &fragment_shader,
                        None,
                    ) {
                        Ok(new_program) => program = new_program,
                        Err(err) => {
                            dbg!(&err);
                        }
                    }

                    let elapsed_time = (last_frame - first_frame).as_secs_f32();

                    let uniforms = uniform! {
                        iTime: elapsed_time,
                        iResolution: i_resolution.pos
                    };

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();

                    platform.prepare_render(ui, &window);
                    let draw_data = imgui.render();
                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");

                    target.finish().expect("Failed to swap buffers");
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        display.resize((new_size.width, new_size.height));
                    }
                    platform.handle_event(imgui.io_mut(), &window, &event);
                    i_resolution = Float3 {
                        pos: [new_size.width as f32, new_size.height as f32, 1.0],
                    };
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => window_target.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            };
        })
        .expect("EventLoop error");
}
