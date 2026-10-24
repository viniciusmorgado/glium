#[macro_use]
extern crate safegl;
use safegl::Surface;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = safegl::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = safegl::backend::glutin::SimpleWindowBuilder::new()
        .with_title("SafeGL tutorial #2")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.25],
        },
    ];
    let vertex_buffer = safegl::VertexBuffer::new(&display, &shape).unwrap();
    let indices = safegl::index::NoIndices(safegl::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        safegl::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &safegl::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            safegl::winit::event::Event::WindowEvent { event, .. } => match event {
                safegl::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                _ => (),
            },
            _ => (),
        })
        .unwrap();
}
