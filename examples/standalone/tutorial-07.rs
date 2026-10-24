#[macro_use]
extern crate safegl;
use safegl::Surface;

#[path = "../../book/tuto-07-teapot.rs"]
mod teapot;

fn main() {
    let event_loop = safegl::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = safegl::backend::glutin::SimpleWindowBuilder::new()
        .with_title("SafeGL tutorial #7")
        .build(&event_loop);

    let positions = safegl::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = safegl::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = safegl::IndexBuffer::new(
        &display,
        safegl::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
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
                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 1.0, 1.0);

                        let matrix = [
                            [0.01, 0.0, 0.0, 0.0],
                            [0.0, 0.01, 0.0, 0.0],
                            [0.0, 0.0, 0.01, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ];

                        target
                            .draw(
                                (&positions, &normals),
                                &indices,
                                &program,
                                &uniform! { matrix: matrix },
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
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                safegl::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
