use gl::types::*;
pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new(width: GLsizei, height: GLsizei, pixels: Vec<u8>) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::TextureStorage2D(id, 1, gl::RGB8, width as GLsizei, height as GLsizei);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TextureSubImage2D(
                id,
                0,                 // level
                0,                 // x
                0,                 // y
                width as GLsizei,  // width
                height as GLsizei, // height
                gl::RGB,           // format
                gl::UNSIGNED_BYTE, // type
                pixels.as_ptr() as *const GLvoid,
            );
        }
        Self { id }
    }
    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}
