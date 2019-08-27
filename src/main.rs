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
mod shader;
mod state;
mod texture;
mod vertex;

use state::State;
use texture::Texture;
use vertex::Vertex;

fn rasterize_glyph(
    glyph_id: u32,
    face: &freetype::face::Face,
) -> (u32, u32, Vec<u8>) {
    face.load_glyph(glyph_id, freetype::face::LoadFlag::DEFAULT).unwrap();
    let glyph = face.glyph();
    glyph.render_glyph(freetype::render_mode::RenderMode::Lcd).unwrap();
    let bitmap = glyph.bitmap();
    // TODO: terrible loop to remove padding/pitch

    ((bitmap.width()/3) as u32, bitmap.rows() as u32, bitmap.buffer().to_vec())
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
        Vertex::new(bottom_left, [1.0, 0.0, 0.0], [0.0, 1.0]),
        Vertex::new(top_left, [0.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new(top_right, [0.0, 0.0, 1.0], [1.0, 0.0]),
        // bottom-right triangle
        Vertex::new(bottom_left, [1.0, 0.0, 0.0], [0.0, 1.0]),
        Vertex::new(bottom_right, [1.0, 1.0, 1.0], [1.0, 1.0]),
        Vertex::new(top_right, [0.0, 0.0, 1.0], [1.0, 0.0]),
    ]
}

fn main() {
    // Init glutin
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new().with_dimensions((1440, 900).into());
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
    let face = lib.new_face(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/fonts/FiraCode-Retina.ttf"
            ), 0).unwrap();
    face.set_pixel_sizes(128,128).unwrap();


    // Create vao
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };

    // Create texture array
    // TODO

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

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for (index, character) in "hello".chars().enumerate() {
            // Render a glyph
            let glyph_id = face.get_char_index(character as usize);
            let (width, height, pixels) = rasterize_glyph(glyph_id, &face);

            // Create a new texture holding the glyph
            let glyph_texture = Texture::new(width as GLsizei, height as GLsizei, pixels);

            // Bind the texture
            unsafe { gl::BindTextureUnit(0, glyph_texture.get_id()) };

            // Create quads
            let mut vertices: Vec<Vertex> = vec![];
            vertices.extend(vertices_for_quad_absolute(
                360 + index as u32 * 100,
                225,
                width as u32,
                height as u32,
                window_width,
                window_height,
            ));

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

        windowed_context.swap_buffers().unwrap();
    }
    unsafe { gl::BindVertexArray(0) };

    // Delete vao
    unsafe { gl::DeleteVertexArrays(1, &vao) };
    // Delete vbo
    unsafe { gl::DeleteBuffers(1, &vbo) };
}
