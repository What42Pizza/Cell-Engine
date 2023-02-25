use crate::prelude::*;
use sdl2::{render::WindowCanvas};





pub fn render(canvas: &mut WindowCanvas, program_data: &mut ProgramData) -> Result<(), ProgramError> {
    let canvas_size = canvas.output_size()?;
    let camera = &program_data.camera;

    //canvas.set_draw_color(Color::RGB(255, 0, 255));
    //canvas.clear();

    let (start_grid_x, start_grid_y) = (program_data.camera.x.floor() as isize, program_data.camera.y.floor() as isize);
    let (end_grid_x, end_grid_y);

    //let start_instant = Instant::now();
    { // draw_ground
        let textures = &program_data.render_data.textures;
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

    { // draw entities
        let textures = &program_data.render_data.textures;
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

    // draw selected entity information
    match program_data.selected_entity {
        EntitySelection::None => {}
        EntitySelection::Cell (entity_id) => draw_cell_information(entity_id, program_data, canvas, canvas_size)?,
        EntitySelection::Food (entity_id) => {}
    }

    // finish
    canvas.present();
    program_data.frame_count += 1;

    Ok(())
}





pub fn draw_entities<T: Entity + AsRef<RawEntity>> (x: usize, y: usize, entities_container: &EntityContainer<T>, camera: &Camera, canvas: &mut WindowCanvas, canvas_size: (u32, u32), textures: &ProgramTextures) -> Result<(), ProgramError> {
    let current_slot = &entities_container.entities_by_pos[x + y * GRID_WIDTH];
    for cell_id in current_slot {
        let entity = entities_container.get(*cell_id).unwrap();
        let raw_entity = entity.as_ref();
        canvas.copy(entity.get_texture(textures), None, get_entity_rect(raw_entity, camera, canvas_size))?;
    }
    Ok(())
}



pub fn get_entity_rect (entity: &RawEntity, camera: &Camera, canvas_size: (u32, u32)) -> Rect {
    let (top_left_x    , top_left_y    ) = fns::convert_grid_to_screen((entity.x - entity.width / 2., entity.y - entity.height / 2.), camera, canvas_size);
    let (bottom_right_x, bottom_right_y) = fns::convert_grid_to_screen((entity.x + entity.width / 2., entity.y + entity.height / 2.), camera, canvas_size);
    Rect::new(top_left_x, top_left_y, (bottom_right_x - top_left_x) as u32, (bottom_right_y - top_left_y) as u32)
}





pub fn draw_cell_information (cell_id: EntityID, program_data: &mut ProgramData, canvas: &mut WindowCanvas, canvas_size: (u32, u32)) -> Result<(), ProgramError> {
    let cell = program_data.cells.get(cell_id).unwrap();
    let main_area = Area::new(canvas_size);
    let menu_area = main_area.get_sub_area(0.0, 0.02, 0., 0.96, 0.02, 0.43);
    render_fns::draw_menu_background(menu_area.to_rect(), canvas)?;

    // "Cell Information"
    let text_pos = menu_area.get_point(0.5, 0.02, 0.0);
    render_fns::draw_text("Cell Information", text_pos, 0.5, canvas_size.1 / 20, canvas, &mut program_data.render_data)?;

    // "Health: "
    let health = (cell.health * 100.).round() / 100.;
    let text_pos = menu_area.get_point(0.05, 0.09, 0.0);
    render_fns::draw_text("Health: ".to_string() + &health.to_string(), text_pos, 0.0, canvas_size.1 / 25, canvas, &mut program_data.render_data)?;

    // "Energy: "
    let energy = (cell.energy * 100.).round() / 100.;
    let text_pos = menu_area.get_point(0.05, 0.14, 0.0);
    render_fns::draw_text("Energy: ".to_string() + &energy.to_string(), text_pos, 0.0, canvas_size.1 / 25, canvas, &mut program_data.render_data)?;

    // "Material: "
    let material = (cell.material * 100.).round() / 100.;
    let text_pos = menu_area.get_point(0.05, 0.19, 0.0);
    render_fns::draw_text("Material: ".to_string() + &material.to_string(), text_pos, 0.0, canvas_size.1 / 25, canvas, &mut program_data.render_data)?;

    let cell_data_area = menu_area.get_sub_area(0.05, 0.25, 0.9, 0.73, 0., 0.);
    render_fns::draw_menu_background(cell_data_area.to_rect(), canvas)?;

    match &cell.raw_cell {
        RawCell::Fat (fat_cell_data) => draw_cell_information_fat(fat_cell_data, cell_data_area, canvas, canvas_size, &mut program_data.render_data)?,
        RawCell::Photosynthesiser => draw_cell_information_photosythesiser(cell_data_area, program_data, canvas, canvas_size)?,
    }

    Ok(())
}





pub fn draw_cell_information_fat (cell_data: &FatCellData, cell_data_area: Area, canvas: &mut WindowCanvas, canvas_size: (u32, u32), render_data: &mut RenderData) -> Result<(), ProgramError> {

    // "Fat Cell"
    let text_pos = cell_data_area.get_point(0.05, 0.19, 0.0);
    render_fns::draw_text("Fat Cell", text_pos, 0.5, canvas_size.1 / 20, canvas, render_data)?;

    Ok(())
}





pub fn draw_cell_information_photosythesiser (cell_data_area: Area, program_data: &ProgramData, canvas: &mut WindowCanvas, canvas_size: (u32, u32)) -> Result<(), ProgramError> {

    Ok(())
}
