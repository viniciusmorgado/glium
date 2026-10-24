#[macro_use]
extern crate safegl;
use safegl::Surface;

fn main() {
    let event_loop = safegl::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = safegl::backend::glutin::SimpleWindowBuilder::new()
        .with_title("SafeGL tutorial #3")
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
        uniform float x_off;

        void main() {
            vec2 pos = position;
            pos.x += x_off;
            gl_Position = vec4(pos, 0.0, 1.0);
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

    let mut t: f32 = 0.0;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                safegl::winit::event::Event::WindowEvent { event, .. } => match event {
                    safegl::winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    safegl::winit::event::WindowEvent::RedrawRequested => {
                        // first we update `t`
                        t += 0.02;
                        let x_off = t.sin() * 0.5;

                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 1.0, 1.0);
                        let uniforms = uniform! { x_off: x_off };
                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &uniforms,
                                &Default::default(),
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    // Because SafeGL doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    safegl::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                safegl::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
