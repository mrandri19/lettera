use gl::types::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    position: [GLfloat; 2],
    color: [GLfloat; 4],
    tex_coords: [GLfloat; 2],
    offset: GLint,
}

impl Vertex {
    pub fn new(
        position: [GLfloat; 2],
        color: [GLfloat; 4],
        tex_coords: [GLfloat; 2],
        offset: GLint,
    ) -> Self {
        Vertex {
            position,
            color,
            tex_coords,
            offset,
        }
    }
    pub fn vertex_specification(vao: GLuint, vbo: GLuint) {
        unsafe {
            // Bind vao and vbo together
            gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, std::mem::size_of::<Self>() as GLint);

            // layout (location = 0) in vec2 in_position;
            let offset = 0;
            let location = 0;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 2, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);

            // layout (location = 1) in vec3 in_color;
            let offset = (2 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 1;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 4, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);

            // layout (location = 2) in vec2 in_tex_coords;
            let offset = (6 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 2;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 2, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);

            // layout (location = 3) in uint in_tex_offsets;
            let offset = (8 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 3;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribIFormat(vao, location, 1, gl::INT, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);
        }
    }
}

pub fn vertices_for_quad_absolute(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
    x_texture: f32,
    y_texture: f32,
    width_texture: f32,
    height_texture: f32,
    offset: GLint,
) -> Vec<Vertex> {
    // Multiplied by two since the OpenGL quadrant goes from -1 to 1 so has length 2
    let x_ss = 2. * x as f32 / window_width as f32;
    let y_ss = 2. * y as f32 / window_height as f32;
    let w_ss = 2. * width as f32 / window_width as f32;
    let h_ss = 2. * height as f32 / window_height as f32;

    let top_left = [-1.0 + x_ss, 1.0 - y_ss];
    let top_right = [-1.0 + x_ss + w_ss, 1.0 - y_ss];
    let bottom_left = [-1.0 + x_ss, 1.0 - (y_ss + h_ss)];
    let bottom_right = [-1.0 + x_ss + w_ss, 1.0 - (y_ss + h_ss)];

    let color = [0.0, 0.0, 0.0, 1.0];
    vec![
        // top-left triangle
        Vertex::new(bottom_left, color, [x_texture, height_texture], offset),
        Vertex::new(top_left, color, [x_texture, y_texture], offset),
        Vertex::new(top_right, color, [width_texture, y_texture], offset),
        // bottom-right triangle
        Vertex::new(bottom_left, color, [x_texture, height_texture], offset),
        Vertex::new(bottom_right, color, [width_texture, height_texture], offset),
        Vertex::new(top_right, color, [width_texture, y_texture], offset),
    ]
}
