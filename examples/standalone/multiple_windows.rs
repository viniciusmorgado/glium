#[macro_use]
extern crate safegl;

use std::collections::HashMap;

use safegl::winit::{event_loop::EventLoop, window::WindowId};
use safegl::{Display, Surface};
use glutin::surface::WindowSurface;
use support::{ApplicationContext, State};

mod support;

struct Application {
    id: i32,
}

// This example shows how to dynamically open new windows.
impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Main Window";

    fn new(_display: &Display<WindowSurface>) -> Self {
        Self { id: 1 }
    }

    /// Here we just fill the window with a solid color based on the id so that we can easily
    /// distinguish them.
    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        let r = if self.id & 1 == 0 { 0.0 } else { 1.0 };
        let g = if self.id & 2 == 0 { 0.0 } else { 1.0 };
        let b = if self.id & 4 == 0 { 0.0 } else { 1.0 };
        frame.clear_color(r, g, b, 1.0);
        frame.finish().unwrap()
    }
}

fn main() {
    let event_loop = EventLoop::builder().build().expect("event loop building");
    // To simplify things we use a single type for both the main and sub windows,
    // since we can easily distinguish them by their id.
    let mut windows: HashMap<WindowId, State<Application>> = HashMap::new();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                // The Resumed/Suspended events are mostly for Android compatiblity since the context can get lost there at any point.
                // For convenience's sake the Resumed event is also delivered on other platforms on program startup.
                safegl::winit::event::Event::Resumed => {
                    // On startup we create the main window and insert it into the HashMap just like subsequent sub windows.
                    let window: State<Application> = State::new(window_target, true);
                    windows.insert(window.window.id(), window);
                }
                safegl::winit::event::Event::Suspended => {
                    windows.clear();
                }
                safegl::winit::event::Event::WindowEvent {
                    event, window_id, ..
                } => {
                    if let Some(state) = windows.get_mut(&window_id) {
                        match event {
                            safegl::winit::event::WindowEvent::MouseInput {
                                state: safegl::winit::event::ElementState::Released,
                                button: safegl::winit::event::MouseButton::Left,
                                ..
                            } => {
                                // This is the main part, where we actually create the new window, enumerate it and
                                // insert it into our HashMap
                                if state.context.id == 1 {
                                    let mut window: State<Application> =
                                        State::new(window_target, true);
                                    window.context.id = windows.len() as i32 + 1;
                                    let title = format!("Window #{}", window.context.id);
                                    window.window.set_title(&title);
                                    windows.insert(window.window.id(), window);
                                } else {
                                    windows.remove(&window_id);
                                }
                            }
                            safegl::winit::event::WindowEvent::RedrawRequested => {
                                state.context.update();
                                state.context.draw_frame(&state.display);
                            }
                            safegl::winit::event::WindowEvent::CloseRequested => {
                                if state.context.id == 1 {
                                    window_target.exit();
                                } else {
                                    windows.remove(&window_id);
                                }
                            }
                            // These two aren't necessary for this particular example, but forgetting especially the Resized
                            // event could lead to stretched images being rendered if you decide to render something more interesting based on this example.
                            safegl::winit::event::WindowEvent::Resized(new_size) => {
                                state.display.resize(new_size.into());
                            }
                            ev => {
                                state.context.handle_window_event(&ev, &state.window);
                            }
                        }
                    }
                }
                _ => (),
            };
        })
        .unwrap();
}
