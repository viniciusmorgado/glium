use std::cell::RefCell;
use std::rc::Rc;

use safegl::backend::web::WebDisplay;
use safegl::index::PrimitiveType;
use safegl::{implement_vertex, program, uniform, Surface};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct Application {
    display: WebDisplay,
    vertex_buffer: safegl::VertexBuffer<Vertex>,
    index_buffer: safegl::IndexBuffer<u16>,
    program: safegl::Program,
}

impl Application {
    fn new(display: WebDisplay) -> Application {
        let vertex_buffer = safegl::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.5],
                    color: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap();

        let index_buffer =
            safegl::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

        let program = program!(&display,
            300 es => {
                vertex: "#version 300 es

                    uniform mat4 matrix;

                    in vec2 position;
                    in vec3 color;

                    out vec3 v_color;

                    void main() {
                        gl_Position = matrix * vec4(position, 0.0, 1.0);
                        v_color = color;
                    }
                ",

                fragment: "#version 300 es
                    precision mediump float;

                    in vec3 v_color;
                    out vec4 f_color;

                    void main() {
                        f_color = vec4(v_color, 1.0);
                    }
                ",
            },
        )
        .unwrap();

        Application {
            display,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn draw(&self, time_seconds: f32) {
        let canvas = self.display.canvas();
        let (width, height) = (canvas.client_width() as u32, canvas.client_height() as u32);
        if (canvas.width(), canvas.height()) != (width, height) {
            self.display.resize((width, height));
        }

        let (sin, cos) = time_seconds.sin_cos();
        let aspect = height as f32 / width.max(1) as f32;
        let uniforms = uniform! {
            matrix: [
                [cos * aspect, sin, 0.0, 0.0],
                [-sin * aspect, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut frame = self.display.draw();
        frame.clear_color(0.07, 0.07, 0.07, 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}

fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // show Rust panics in the browser console
    std::panic::set_hook(Box::new(|info| web_sys::console::error_1(&info.to_string().into())));

    let canvas = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("safegl-canvas"))
        .ok_or_else(|| JsValue::from_str("missing #safegl-canvas element"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let display = WebDisplay::from_canvas(canvas).map_err(|err| JsValue::from_str(&err.to_string()))?;
    let application = Application::new(display);

    // draw on every animation frame; the closure keeps itself alive
    let frame_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let next_frame = frame_callback.clone();
    *frame_callback.borrow_mut() = Some(Closure::new(move |timestamp_ms: f64| {
        application.draw((timestamp_ms / 1000.0) as f32);
        request_animation_frame(next_frame.borrow().as_ref().unwrap());
    }));
    request_animation_frame(frame_callback.borrow().as_ref().unwrap());

    Ok(())
}
