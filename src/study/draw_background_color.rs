use glium::{
    glutin::surface::WindowSurface,
    Surface,
    Display,
};

pub fn rendering_background_color(display: &Display<WindowSurface>) {
    // Start rendering by creating a new target
    let mut target = display.draw();
    // Which we fill with an opaque blue color
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    // By finishing the target swap buffers and thereby make it visible on the window
    target.finish().unwrap();
}