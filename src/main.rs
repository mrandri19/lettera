// TODO: understand this, but it's a fucking genious idea.
// the vertex shader becomes more complicated though, it needs some
// benchmarking
// https://ourmachinery.com/post/ui-rendering-using-primitive-buffers/

// TODO
// support hidpi, right now on a 4k screen it renders on the surface as
// if it was a 1920 screen, or maybe I'm just creating the texture sizes wrong

// TODO
// * find out how to use harfbuzz-rs
// * look at multi draw and glsl subroutines

use gl::types::*;

use freetype::Library;

mod debug_message_callback;
mod face_bbox;
mod program;
mod rasterize_glyph;
mod shader;
mod state;
mod texture_atlas;
mod vertex;

use crate::rasterize_glyph::rasterize_glyph;
use crate::state::State;
use crate::texture_atlas::TextureAtlas;
use crate::vertex::Vertex;

fn draw_frame(
    lines: &Vec<String>,
    face: &freetype::face::Face,
    vao: GLuint,
    vbo: GLuint,
    program: GLuint,
    window_width: u32,
    window_height: u32,
    line_height: u32,
    texture_atlas: &mut TextureAtlas,
) {
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    let mut pen_position_x = 0;
    let mut pen_position_y = line_height as i32;

    let mut vertices: Vec<Vertex> = vec![];
    for line in lines
        .iter()
        .take((window_height / line_height + 1) as usize)
    {
        for character in line.chars() {
            let glyph_index = face.get_char_index(character as usize);

            let (
                glyph_width,
                glyph_height,
                offset,
                left_bearing,
                top_bearing,
                advance,
                width_texture,
                height_texture,
            ) = texture_atlas.get(glyph_index).unwrap_or_else(|| {
                texture_atlas.insert(glyph_index, rasterize_glyph(glyph_index, &face));
                texture_atlas.get(glyph_index).unwrap()
            });

            vertices.extend(vertices_for_quad_absolute(
                std::cmp::max(0, pen_position_x + left_bearing) as u32,
                std::cmp::max(0, pen_position_y - top_bearing) as u32,
                glyph_width as u32,
                glyph_height as u32,
                window_width,
                window_height,
                0.0,
                0.0,
                width_texture,
                height_texture,
                offset as GLint,
            ));

            pen_position_x += advance;
        }
        pen_position_x = 0;
        pen_position_y += line_height as i32;
    }

    // Load quads into vbo
    unsafe {
        // TODO(optimization): reallocating the buffer every frame is bad,
        //  or maybe not so much, look more into it when we have benchmarks
        gl::NamedBufferData(
            vbo,
            (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        )
    };

    // Setup vao for quad drawing
    Vertex::vertex_specification(vao, vbo);

    // Bind the texture
    let texture_unit = 0;
    unsafe { gl::BindTextureUnit(texture_unit, texture_atlas.get_id()) };
    unsafe {
        let sampler_loc =
            gl::GetUniformLocation(program, std::ffi::CString::new("tex").unwrap().as_ptr());
        gl::Uniform1i(sampler_loc, texture_unit as i32);
    };

    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as GLsizei);
    }
}

fn vertices_for_quad_absolute(
    x: u32,
    y: u32,
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

    let color = [0.0, 0.0, 0.0];
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

fn main() {
    // Init glutin
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new().with_dimensions((750, 200).into());
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let window = windowed_context.window();
    let context = windowed_context.context();

    // Load OpenGL
    gl::load_with(|s| context.get_proc_address(s) as *const _);

    // Enable debug outuput
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(debug_message_callback::callback, std::ptr::null());
    }

    // Create state
    let mut state = State::new(window.get_inner_size().unwrap());

    // Create vbo
    let mut vbo: GLuint = 0;
    unsafe { gl::CreateBuffers(1, &mut vbo) };

    // Create and use shader program
    let program = program::Program::new().unwrap();
    program.use_();

    // Load font
    let size = 16;
    let lib = Library::init().unwrap();
    lib.set_lcd_filter(freetype::LcdFilter::LcdFilterDefault)
        .unwrap();
    let face = lib
        .new_face(
            concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/UbuntuMono.ttf"),
            0,
        )
        .unwrap();
    face.set_char_size(size * 64, 0, 72, 0).unwrap();
    let line_height = (size as f64 * 1.35) as u32;

    // Get face bbox
    let (max_face_width_pixels, max_face_height_pixels) = face_bbox::face_bbox(&face);

    // Create vao
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };

    // Create texture atlas
    let mut texture_atlas = TextureAtlas::new(
        max_face_width_pixels as GLsizei,
        max_face_height_pixels as GLsizei,
        512,
    );

    // Enable blending
    unsafe { gl::Enable(gl::BLEND) };
    unsafe {
        gl::BlendFunc(gl::SRC1_COLOR, gl::ONE_MINUS_SRC1_COLOR);
    };

    // Read file
    let file_name = std::env::args().nth(1).unwrap_or("src/main.rs".to_string());
    let lines = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Bind vertex array buffer before drawing
    unsafe { gl::BindVertexArray(vao) };
    // Main loop
    while state.is_running() {
        el.poll_events(|e| state.handle_event(e));

        // Update viewport on resize
        if state.should_update_viewport() {
            let logical_size = state.get_logical_size();
            let (width, height): (u32, u32) =
                logical_size.to_physical(window.get_hidpi_factor()).into();
            unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei) };
        }

        // Get window size
        let (window_width, window_height): (u32, u32) = window.get_inner_size().unwrap().into();

        draw_frame(
            &lines,
            &face,
            vao,
            vbo,
            program.get_id(),
            window_width,
            window_height,
            line_height as u32,
            &mut texture_atlas,
        );

        windowed_context.swap_buffers().unwrap();
    }
    unsafe { gl::BindVertexArray(0) };

    // Delete vao
    unsafe { gl::DeleteVertexArrays(1, &vao) };
    // Delete vbo
    unsafe { gl::DeleteBuffers(1, &vbo) };
}
