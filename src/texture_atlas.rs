use gl::types::*;
use std::collections::HashMap;

type GlyphIndex = u32;
type GlyphDescription = (u32, u32, usize, i32, i32, i32, f32, f32);

pub struct TextureAtlas {
    id: GLuint,
    glyph_dimensions: HashMap<GlyphIndex, GlyphDescription>,
    size: usize,
    capacity: usize,
    width: GLsizei,
    height: GLsizei,
}

impl TextureAtlas {
    pub fn new(width: GLsizei, height: GLsizei, capacity: usize) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut id);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TextureStorage3D(
                id,
                1, // mipLevelCount
                gl::RGB8,
                width,
                height,
                capacity as GLint,
            );
        }
        Self {
            id,
            glyph_dimensions: HashMap::new(),
            size: 0,
            capacity,
            width,
            height,
        }
    }
    pub fn get_id(&self) -> GLuint {
        self.id
    }
    pub fn get(&mut self, glyph_index: u32) -> Option<GlyphDescription> {
        if let Some(glyph_description) = self.glyph_dimensions.get(&glyph_index) {
            Some(glyph_description.clone())
        } else {
            None
        }
    }
    pub fn insert(
        &mut self,
        glyph_index: u32,
        rasterized_glyph: (u32, u32, Vec<u8>, i32, i32, i32),
    ) {
        assert!(self.size < self.capacity);
        let (glyph_width, glyph_height, pixels, left_bearing, top_bearing, advance) =
            rasterized_glyph;
        self.glyph_dimensions.insert(
            glyph_index,
            (
                glyph_width,
                glyph_height,
                self.size,
                left_bearing,
                top_bearing,
                advance,
                glyph_width as f32 / self.width as f32,
                glyph_height as f32 / self.height as f32,
            ),
        );

        unsafe {
            gl::TextureSubImage3D(
                self.id,                 // texture
                0,                       // level
                0,                       // x
                0,                       // y
                self.size as GLint,      // z
                glyph_width as GLsizei,  // width
                glyph_height as GLsizei, // height
                1,                       // depth
                gl::RGB,                 // format
                gl::UNSIGNED_BYTE,       // type
                pixels.as_ptr() as *const GLvoid,
            )
        };
        unsafe {
            gl::GenerateTextureMipmap(self.id);
        }

        self.size += 1;
    }
}

impl Drop for TextureAtlas {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}
