use super::Vertex;
use glium::{
    glutin::surface::WindowSurface, Display, Program, Surface, VertexBuffer
};


pub fn build_vertex_buffer(display: &Display<WindowSurface>) -> VertexBuffer<Vertex> {
    // In graphics programming all the shapes are build using triangles
    // A triangle is made of three vertices, 
    // A shape is just a collection of vertices linked together to form triangles

    let vertex1 = Vertex::new([-0.5, -0.5]);
    let vertex2 = Vertex::new([0.0, 0.5]);
    let vertex3 = Vertex::new([0.5, -0.25]);
    let shape = vec![vertex1, vertex2, vertex3];

    
    // A vertex buffer upload a shape to the memory of the GPU
    let vertex_buffer = 
        glium::VertexBuffer::new(
            display,
            &shape
        ).unwrap();
    
    return vertex_buffer;
}

pub fn build_program_payload(display: &Display<WindowSurface>) -> Program {
    // When the GPU draw a shape, it will first execute a vertex shader once for each vertex 
    // A vertex shader tells the GPU what the screen coordinates of each vertex are going to be
    // and determines which pixels of the screen are inside the shape
    // then is execute a fragment shader for each of these pixels 
    // A fragment shader tells the GPU what color each pixel inside the shpe should be
    
    // To do both of those processes is necessary using GLSL programming language

    // The vertex_shader script is called once per vertex
    // "#version 140" line is where is defined the version of GLSL this shader is written in
    // "vec2 position;" line declare that is expected to be given an attribute named "position" of type vec2 ([f32;2])
    // "gl_Position = vec4(position, 0.0, 1.0);" line tell OpenGL what the final position of our vertex
    let vertex_shader = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    
    // The fragment_shader GLSL script works similar to the vertex shader script
    // the only diference is that the "color = vec4(1.0, 0.0, 0.0, 1.0);" line
    // defines the rgb color of the pixels inside the triangle
    let fragment_shader = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    // Now we create a draw settings payload call program
    // that contains the vertex_shader and fragment_shader configurations
    let program = 
        glium::Program::from_source(
            display,
            vertex_shader,
            fragment_shader,
            None
        ).unwrap();
    
    return program;
}

pub fn draw_shape_vertex_buffer(
    display: &Display<WindowSurface>,
    vertex_buffer: &VertexBuffer<Vertex>,
    program: &Program,
) {
    // The indice defines the vertex render order
    let indices = 
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(
        vertex_buffer,
        &indices,
        program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
    target.finish().unwrap();
}