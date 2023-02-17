use crate::prelude::*;
use sdl2::render::{WindowCanvas, Texture};



pub fn render(canvas: &mut WindowCanvas, program_data: &mut ProgramData) -> Result<(), ProgramError> {
    let textures = &program_data.textures;
    let canvas_size = canvas.output_size()?;
    let camera = &program_data.camera;

    //canvas.set_draw_color(Color::RGB(255, 0, 255));
    //canvas.clear();

    let (start_grid_x, start_grid_y) = (program_data.camera.x.floor() as isize, program_data.camera.y.floor() as isize);
    let (end_grid_x, end_grid_y);

    let start_instant = Instant::now();
    'draw_ground: {
        let zoom = program_data.camera.zoom;
        let start_curr_x = fns::convert_single_grid_to_screen(start_grid_x as f64, camera.x, zoom, canvas_size);
        let start_next_x = fns::convert_single_grid_to_screen(start_grid_x as f64 + 1., camera.x, zoom, canvas_size);
        let mut curr_grid_y = start_grid_y;
        let mut curr_screen_y = fns::convert_single_grid_to_screen(curr_grid_y as f64, camera.y, zoom, canvas_size);
        let mut next_screen_y = fns::convert_single_grid_to_screen(curr_grid_y as f64 + 1., camera.y, zoom, canvas_size);
        'y: loop {
            let mut curr_grid_x = start_grid_x;
            let mut curr_screen_x = start_curr_x;
            let mut next_screen_x = start_next_x;
            'x: loop {
                let dst = Rect::new(curr_screen_x, curr_screen_y, (next_screen_x - curr_screen_x) as u32, (next_screen_y - curr_screen_y) as u32);
                let texture = if curr_grid_x < 0 || curr_grid_y < 0 || curr_grid_x >= GRID_WIDTH as isize || curr_grid_y >= GRID_HEIGHT as isize {
                    &textures.black_ground
                } else {
                    &textures.ground
                };
                canvas.copy(texture, None, dst)?;
                curr_grid_x += 1;
                curr_screen_x = next_screen_x;
                next_screen_x = fns::convert_single_grid_to_screen(curr_grid_x  as f64+ 1., camera.x, zoom, canvas_size);
                if curr_screen_x > canvas_size.0 as i32 {break 'x;}
            }
            curr_grid_y += 1;
            curr_screen_y = next_screen_y;
            next_screen_y = fns::convert_single_grid_to_screen(curr_grid_y as f64 + 1., camera.y, zoom, canvas_size);
            if curr_screen_y > canvas_size.1 as i32 {
                end_grid_x = curr_grid_x;
                end_grid_y = curr_grid_y;
                break 'y;
            }
        }
    }
    //println!("{}", start_instant.elapsed().as_micros());

    //let (start_grid_x, start_grid_y) = (start_grid_x as isize, start_grid_y as isize);
    //let (end_grid_x, end_grid_y) = (end_grid_x as isize, end_grid_y as isize);

    'draw_entities: {
        let start_grid_x = (start_grid_x - 1).max(0);
        let start_grid_y = (start_grid_y - 1).max(0);
        let end_grid_x = (end_grid_x + 1).min(GRID_WIDTH  as isize - 1);
        let end_grid_y = (end_grid_y + 1).min(GRID_HEIGHT as isize - 1);
        for y in start_grid_y..=end_grid_y {
            for x in start_grid_x..=end_grid_x {
                let (x, y) = (x as usize, y as usize);
                draw_entities(x, y, &program_data.cells, camera, canvas, canvas_size, textures)?;
                draw_entities(x, y, &program_data.food, camera, canvas, canvas_size, textures)?;
            }
        }
    }

    // finish
    canvas.present();
    program_data.frame_count += 1;

    Ok(())
}





pub fn draw_entities<T: Entity> (x: usize, y: usize, entities_container: &EntityContainer<T>, camera: &Camera, canvas: &mut WindowCanvas, canvas_size: (u32, u32), textures: &ProgramTextures) -> Result<(), ProgramError> {
    let current_slot = &entities_container.entities_by_pos[x + y * GRID_WIDTH];
    for cell_id in current_slot {
        let entity = entities_container.master_list.get(cell_id).unwrap();
        canvas.copy(entity.get_texture(textures), None, get_entity_rect(entity.as_ref(), camera, canvas_size))?;
    }
    Ok(())
}



pub fn get_entity_rect (entity: &RawEntity, camera: &Camera, canvas_size: (u32, u32)) -> Rect {
    let (top_left_x    , top_left_y    ) = fns::convert_grid_to_screen((entity.x - entity.width / 2., entity.y - entity.height / 2.), camera, canvas_size);
    let (bottom_right_x, bottom_right_y) = fns::convert_grid_to_screen((entity.x + entity.width / 2., entity.y + entity.height / 2.), camera, canvas_size);
    Rect::new(top_left_x, top_left_y, (bottom_right_x - top_left_x) as u32, (bottom_right_y - top_left_y) as u32)
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
