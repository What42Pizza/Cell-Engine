use crate::prelude::*;
use sdl2::{render::WindowCanvas};





pub fn draw_menu_background (rect: Rect, canvas: &mut WindowCanvas) -> Result<(), ProgramError> {

    let border_width = (canvas.output_size()?.1 as f64 * 0.004).round() as u32;

    // main area
    canvas.set_draw_color(Color::RGB(22, 22, 29));
    canvas.fill_rect(rect)?;

    // edges
    canvas.set_draw_color(Color::RGB(16, 16, 20));
    canvas.fill_rect(Rect::new(rect.x, rect.y, rect.w as u32, border_width))?; // top
    canvas.fill_rect(Rect::new(rect.x, rect.y + rect.h - border_width as i32, rect.w as u32, border_width))?; // bottom
    canvas.fill_rect(Rect::new(rect.x, rect.y, border_width, rect.h as u32))?; // left
    canvas.fill_rect(Rect::new(rect.x + rect.w - border_width as i32, rect.y, border_width, rect.h as u32))?; // right

    Ok(())
}





pub fn draw_text (text: impl AsRef<str>, text_pos: (i32, i32), x_align: f64, size: u32, canvas: &mut WindowCanvas, render_data: &mut RenderData) -> Result<(), ProgramError> {
    let text = text.as_ref();
    let glyphs = text.chars()
        .map(|c| {
            render_data.font
                .glyph_id(c)
                .with_scale(PxScale::from(size as f32))
        })
        .collect::<Vec<Glyph>>();

    // ensure glyphs are rendered
    for glyph in &glyphs {
        ensure_glyph_is_rendered(glyph, render_data)?;
    }

    // get text positioning
    let mut glyph_positions = Vec::with_capacity(glyphs.len());
    let mut current_x = 0;
    let scaled_font = render_data.font.as_scaled(PxScale::from(size as f32));
    for glyph in &glyphs {
        glyph_positions.push(current_x);
        current_x += scaled_font.h_advance(glyph.id) as u32;
    }
    glyph_positions.push(current_x);
    let width = current_x;
    let left_x = text_pos.0 - (width as f64 * x_align) as i32;

    // place text
    for (i, glyph) in glyphs.iter().enumerate() {
        let texture = render_data.glyph_cache.get(&HashableGlyph::from_glyph(glyph)).unwrap();
        let texture_size = fns::get_texture_size(&texture.texture);
        let dst = Rect::new(left_x + glyph_positions[i] as i32 + texture.origin_x, text_pos.1 + texture.origin_y + size as i32, texture_size.0, texture_size.1);
        canvas.copy(&texture.texture, None, dst)?;
    }

    Ok(())
}



pub fn ensure_glyph_is_rendered (glyph: &Glyph, render_data: &mut RenderData) -> Result<(), ProgramError> {

    // return if already rendered
    let hashable_glyph = HashableGlyph::from_glyph(glyph);
    if render_data.glyph_cache.contains_key(&hashable_glyph) {return Ok(());}

    let texture = render_fns::render_glyph(glyph, render_data)?;

    // cache
    render_data.glyph_cache.insert(hashable_glyph, texture);

    Ok(())
}





pub fn render_glyph<'a> (glyph: &Glyph, render_data: &mut RenderData<'a>) -> Result<GlyphTexture<'a>, ProgramError> {

    // get render data
    let glyph_outline = match render_data.font.outline_glyph(glyph.clone()) {
        Some(v) => v,
        None => {
            let texture = fns::create_texture(glyph.scale.x as u32, glyph.scale.y as u32, render_data.texture_creator);
            return Ok(GlyphTexture {texture, origin_x: 0, origin_y: 0});
        },
    };
    let min = glyph_outline.px_bounds().min;
    let max = glyph_outline.px_bounds().max;
    let (width, height) = ((max.x - min.x) as usize, (max.y - min.y) as usize);

    // render to vec
    let mut pixel_data = Vec::with_capacity(width * height);
    glyph_outline.draw(|x, y, c| {
        pixel_data.push(255);
        pixel_data.push(255);
        pixel_data.push(255);
        pixel_data.push((c * 255.) as u8);
    });

    // vec -> texture
    let mut texture = fns::create_texture(width as u32, height as u32, render_data.texture_creator);
    texture.update(None, &pixel_data, width * 4)?;

    Ok(GlyphTexture {
        texture,
        origin_x: min.x as i32,
        origin_y: min.y as i32,
    })
}





pub fn clamp_to_section (rect: &Rect, section: &Rect) -> (Rect, Rect) {
    let (lx, ly) = (rect.x, rect.y);
    let (width, height) = (rect.width(), rect.height());
    let (hx, hy) = (lx + width as i32, ly + height as i32);
    let (section_lx, section_ly) = (section.x(), section.y());
    let (section_width, section_height) = (section.width(), section.height());

    let shown_lx = lx.max(0);
    let shown_ly = ly.max(0);
    let shown_hx = hx.min(section_width as i32);
    let shown_hy = hy.min(section_height as i32);
    let src_lx = shown_lx - lx;
    let src_ly = shown_ly - ly;
    let src_hx = shown_hx - hx + width as i32;
    let src_hy = shown_hy - hy + height as i32;

    let src = Rect::new(src_lx, src_ly, (src_hx - src_lx) as u32, (src_hy - src_ly) as u32);
    let dest = Rect::new(shown_lx + section_lx, shown_ly + section_ly, (shown_hx - shown_lx) as u32, (shown_hy - shown_ly) as u32);
    (src, dest)
}
