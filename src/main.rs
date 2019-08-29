// TODO: understand this, but it's  a fucking genious idea.
// the vertex shader becomes more complicated though, it needs some
// benchmarking
// https://ourmachinery.com/post/ui-rendering-using-primitive-buffers/

// TODO: finish
// * we need LCD filtering, either font-kit merges a patch quickly or I'll fork it
// * find out how to represent a glyph + glyph texture atlasing
// * find out how to use harfbuzz-rs

// TODO: look at multi draw and glsl subroutines

use gl::types::*;

use freetype::Library;

mod debug_message_callback;
mod program;
mod rasterize_glyph;
mod shader;
mod state;
mod texture;
mod vertex;

use crate::rasterize_glyph::rasterize_glyph;
use crate::state::State;
use crate::texture::Texture;
use crate::vertex::Vertex;

fn draw_frame(
    face: &freetype::face::Face,
    vao: GLuint,
    vbo: GLuint,
    window_width: u32,
    window_height: u32,
) {
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    let mut pen_position_x = 10;
    let pen_position_y = 150;

    for character in "gl::Clear(gl::COLOR_BUFFER_BIT);".chars() {
        let glyph_index = face.get_char_index(character as usize);

        let (glyph_width, glyph_height, glyph_pixels, left_bearing, top_bearing, advance) =
            rasterize_glyph(glyph_index, &face);

        if character == ' ' {
            pen_position_x += advance;
            continue;
        }

        // Create a new texture holding the glyph
        let glyph_texture = Texture::new(
            glyph_width as GLsizei,
            glyph_height as GLsizei,
            glyph_pixels,
        );

        // Bind the texture
        unsafe { gl::BindTextureUnit(0, glyph_texture.get_id()) };

        let mut vertices: Vec<Vertex> = vec![];
        vertices.extend(vertices_for_quad_absolute(
            (pen_position_x + left_bearing) as u32,
            (pen_position_y - top_bearing) as u32,
            glyph_width as u32,
            glyph_height as u32,
            window_width,
            window_height,
        ));

        pen_position_x += advance;

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

        // Draw quads
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as GLsizei);
        }
    }
}

fn vertices_for_quad_absolute(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
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

    vec![
        // top-left triangle
        Vertex::new(bottom_left, [0.0, 0.0, 0.0], [0.0, 1.0]),
        Vertex::new(top_left, [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new(top_right, [0.0, 0.0, 0.0], [1.0, 0.0]),
        // bottom-right triangle
        Vertex::new(bottom_left, [0.0, 0.0, 0.0], [0.0, 1.0]),
        Vertex::new(bottom_right, [0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new(top_right, [0.0, 0.0, 0.0], [1.0, 0.0]),
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
        gl::DebugMessageCallback(debug_message_callback::callback, std::ptr::null())
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
    let lib = Library::init().unwrap();
    lib.set_lcd_filter(freetype::LcdFilter::LcdFilterDefault)
        .unwrap();
    let face = lib
        .new_face(
            concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/UbuntuMono.ttf"),
            0,
        )
        .unwrap();
    let size: isize = std::env::args().nth(1).unwrap().parse().unwrap();
    face.set_char_size(size * 64, 0, 72, 0).unwrap();

    // Create vao
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };

    // Create texture array
    // TODO

    // Enable blending
    unsafe { gl::Enable(gl::BLEND) };
    unsafe {
        gl::BlendFunc(gl::SRC1_COLOR, gl::ONE_MINUS_SRC1_COLOR);
    };

    // Bind vertex array buffer before drawing
    unsafe { gl::BindVertexArray(vao) };
    // Main loop
    while state.is_running() {
        // Handle events
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

        draw_frame(&face, vao, vbo, window_width, window_height);

        windowed_context.swap_buffers().unwrap();
    }
    unsafe { gl::BindVertexArray(0) };

    // Delete vao
    unsafe { gl::DeleteVertexArrays(1, &vao) };
    // Delete vbo
    unsafe { gl::DeleteBuffers(1, &vbo) };
}
