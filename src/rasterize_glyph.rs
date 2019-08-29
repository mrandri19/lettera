pub fn rasterize_glyph(
    glyph_id: u32,
    face: &freetype::face::Face,
) -> (u32, u32, Vec<u8>, i32, i32, i32) {
    face.load_glyph(
        glyph_id,
        freetype::face::LoadFlag::DEFAULT | freetype::face::LoadFlag::TARGET_LIGHT,
    )
    .unwrap();

    let glyph = face.glyph();
    glyph
        .render_glyph(freetype::render_mode::RenderMode::Lcd)
        .unwrap();

    let bitmap = glyph.bitmap();
    let padded_buffer = bitmap.buffer();

    let mut buffer: Vec<u8> = vec![0; (bitmap.width() * bitmap.rows()) as usize];
    for y in 0..bitmap.rows() {
        for x in 0..bitmap.width() {
            buffer[(y * bitmap.width() + x) as usize] =
                padded_buffer[(y * bitmap.pitch() + x) as usize];
        }
    }

    // TODO(andrea): why when comparing with sublime or vscode the ( and ) are much larger?

    let pixel_width = (bitmap.width() / 3) as u32;
    let pixel_height = bitmap.rows() as u32;
    let left_bearing = glyph.bitmap_left();
    let top_bearing = glyph.bitmap_top();
    let advance = glyph.advance().x / 64;

    assert_eq!(glyph.advance().x % 64, 0);
    assert_eq!(bitmap.width() % 3, 0);

    (
        pixel_width,
        pixel_height,
        buffer,
        left_bearing,
        top_bearing,
        advance as i32,
    )
}
