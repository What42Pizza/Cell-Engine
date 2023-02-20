use sdl2::render::WindowCanvas;

use crate::prelude::*;





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
