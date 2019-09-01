pub fn face_bbox(face: &freetype::Face) -> (u32, u32) {
    let raw_face = face.raw();
    let bbox = raw_face.bbox;
    let max_face_width_font_units = bbox.xMax - bbox.xMin;
    let max_face_height_font_units = bbox.yMax - bbox.yMin;

    let x_scale = unsafe { (*raw_face.size).metrics.x_scale };
    let y_scale = unsafe { (*raw_face.size).metrics.y_scale };

    let max_face_width_pixels = max_face_width_font_units * x_scale / 0x10000 / 64;
    let max_face_height_pixels = max_face_height_font_units * y_scale / 0x10000 / 64;
    (max_face_width_pixels as u32, max_face_height_pixels as u32)
}
