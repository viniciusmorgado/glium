use safegl::Surface;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = safegl::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = safegl::backend::glutin::SimpleWindowBuilder::new()
        .with_title("SafeGL tutorial #1")
        .build(&event_loop);

    // Start rendering by creating a new frame
    let mut frame = display.draw();
    // Which we fill with an opaque blue color
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    // By finishing the frame swap buffers and thereby make it visible on the window
    frame.finish().unwrap();

    // Now we wait until the program is closed
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                safegl::winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    safegl::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    _ => (),
                },
                _ => (),
            };
        })
        .unwrap();
}
