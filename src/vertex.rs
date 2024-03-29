use gl::types::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    position: [GLfloat; 2],
    color: [GLfloat; 3],
    tex_coords: [GLfloat; 2],
}

impl Vertex {
    pub fn new(position: [GLfloat; 2], color: [GLfloat; 3], tex_coords: [GLfloat; 2]) -> Self {
        Vertex {
            position,
            color,
            tex_coords,
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
            gl::VertexArrayAttribFormat(vao, location, 3, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);

            // layout (location = 2) in vec2 in_color;
            let offset = (5 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 2;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 2, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);
        }
    }
}
