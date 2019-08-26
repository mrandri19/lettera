use gl::types::*;

use crate::shader::Shader;
use std::ffi::CString;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new() -> Result<Self, String> {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.vert"
            )))
            .unwrap(),
            gl::VERTEX_SHADER,
        )?;
        let fragment_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.frag"
            )))
            .unwrap(),
            gl::FRAGMENT_SHADER,
        )?;

        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader.id());
            gl::AttachShader(program, fragment_shader.id());
            gl::LinkProgram(program);
            gl::DetachShader(program, vertex_shader.id());
            gl::DetachShader(program, fragment_shader.id());
        }

        Ok(Program { id: program })
    }
    pub fn use_(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
