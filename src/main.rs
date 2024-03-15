#[macro_use]
extern crate glium;
extern crate winit;

mod study;

fn main() {
    // First start by building the EventLoop
    // The EventLoop is a struct with attributs to a loop runing in paralellism  
    let event_loop = 
        winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");


    // Seccond we build 2 struct 
    // - window: corresponds to the widow where the program gonna run
    // - display: manager what is being shown on the window
    let (_window, display) =
        glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Physics")
        .build(&event_loop);

    
    // Rendering elements
    study::rendering_background_color(&display);
    let vertex_buffer = study::build_vertex_buffer(&display);
    let program = study::build_program_payload(&display);
    study::draw_shape_vertex_buffer(&display, &vertex_buffer, &program);
    

    // Finally we start the event loop thread 
    // This process runs a closure in loop multithreading 
    // The closure running in loop on multithreading update
    // The event loop should run on the main thread to work
    event_loop.run(move |event, window_target| {
        // The event closure parameter loads information about window events
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window
                // or request the program to quit via the taskbar.
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();
}

