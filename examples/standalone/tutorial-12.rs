#[macro_use]
extern crate safegl;
use safegl::Surface;

#[path = "../../book/tuto-07-teapot.rs"]
mod teapot;

mod support;
use support::view_matrix;

fn main() {
    let event_loop = safegl::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = safegl::backend::glutin::SimpleWindowBuilder::new()
        .with_title("SafeGL tutorial #12")
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
            #version 150

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                mat4 modelview = view * model;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 150

            in vec3 v_normal;
            out vec4 color;
            uniform vec3 u_light;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(u_light));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#;

    let program =
        safegl::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            safegl::winit::event::Event::WindowEvent { event, .. } => match event {
                safegl::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                safegl::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                    let model = [
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 2.0, 1.0f32]
                    ];

                    let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

                    let perspective = {
                        let (width, height) = target.get_dimensions();
                        let aspect_ratio = height as f32 / width as f32;

                        let fov: f32 = 3.141592 / 3.0;
                        let zfar = 1024.0;
                        let znear = 0.1;

                        let f = 1.0 / (fov / 2.0).tan();

                        [
                            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                            [         0.0         ,     f ,              0.0              ,   0.0],
                            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                        ]
                    };

                    let light = [-1.0, 0.4, 0.9f32];

                    let params = safegl::DrawParameters {
                        depth: safegl::Depth {
                            test: safegl::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        //backface_culling: safegl::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };

                    target.draw((&positions, &normals), &indices, &program,
                                &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                                &params).unwrap();
                    target.finish().unwrap();
                },
                // Because SafeGL doesn't know about windows we need to resize the display
                // when the window's size has changed.
                safegl::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            safegl::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}
