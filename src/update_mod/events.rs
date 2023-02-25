use crate::prelude::*;
use sdl2::{event::Event, keyboard::Keycode, render::WindowCanvas, mouse::{MouseState, MouseButton}};



pub fn process_events (program_data: &mut ProgramData, events_data: EventsData, canvas: &WindowCanvas) -> Result<(), ProgramError> {

    for event in events_data.list.iter() {
        match event {
            Event::Quit {..} => program_data.exit = true,
            Event::KeyDown {keycode: Some(keycode), ..} => handle_key_down(program_data, *keycode),
            Event::KeyUp {keycode: Some(keycode), ..} => handle_key_up(program_data, *keycode),
            Event::MouseWheel {y, ..} => process_mouse_wheel(program_data, *y, &events_data.mouse_state, canvas)?,
            Event::MouseButtonDown {mouse_btn, x, y, ..} => process_mouse_click(program_data, *mouse_btn, *x, *y, canvas)?,
            _ => {}
        }
    }

    Ok(())
}





pub fn handle_key_down (program_data: &mut ProgramData, keycode: Keycode) {
    match keycode {

        Keycode::Escape => handle_esc_pressed(program_data),

        Keycode::W if program_data.key_is_pressed(Keycode::LCtrl) => {
            program_data.exit = true;
        }

        _ => {program_data.keys_pressed.insert(keycode, ());},

    }
}



pub fn handle_key_up (program_data: &mut ProgramData, keycode: Keycode) {
    program_data.keys_pressed.remove(&keycode);
}



pub fn handle_esc_pressed (program_data: &mut ProgramData) {

    // remove selected entity
    if program_data.selected_entity != EntitySelection::None {
        program_data.selected_entity = EntitySelection::None;
        return;
    }

}





pub fn process_mouse_click (program_data: &mut ProgramData, mouse_button: MouseButton, x: i32, y: i32, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    
    let clicked_item = get_screen_item_at_pos(x, y, program_data, canvas)?;
    match clicked_item {
        ScreenItem::None => program_data.selected_entity = EntitySelection::None,
        ScreenItem::Cell (entity_id) => program_data.selected_entity = EntitySelection::Cell(entity_id),
        ScreenItem::Food (entity_id) => program_data.selected_entity = EntitySelection::Food(entity_id),
    }

    Ok(())
}



pub fn process_mouse_wheel (program_data: &mut ProgramData, y: i32, mouse_state: &MouseState, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    let canvas_size = canvas.output_size()?;
    let mouse_pos = (mouse_state.x(), mouse_state.y());
    let start_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.zoom *= SCROLL_SPEED.pow(y as f64);
    program_data.camera.zoom = program_data.camera.zoom.max(MAX_ZOOM_OUT);
    let end_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.x -= end_grid_pos.0 - start_grid_pos.0;
    program_data.camera.y -= end_grid_pos.1 - start_grid_pos.1;
    Ok(())
}





pub enum ScreenItem {
    None,
    Cell (EntityID),
    Food (EntityID),
}

pub fn get_screen_item_at_pos (x: i32, y: i32, program_data: &ProgramData, canvas: &WindowCanvas) -> Result<ScreenItem, ProgramError> {

    let map_pos = fns::convert_screen_to_grid((x, y), &program_data.camera, canvas.output_size()?);
    let grid_pos = (map_pos.0 as usize, map_pos.1 as usize);

    if let Some(entity_id) = get_entity_at_pos(grid_pos, map_pos, &program_data.cells) {
        return Ok(ScreenItem::Cell(entity_id));
    }
    if let Some(entity_id) = get_entity_at_pos(grid_pos, map_pos, &program_data.food) {
        return Ok(ScreenItem::Food(entity_id));
    }

    Ok(ScreenItem::None)
}



pub fn get_entity_at_pos<T: Entity + AsRef<RawEntity>> (grid_pos: (usize, usize), map_pos: (f64, f64), entities: &EntityContainer<T>) -> Option<EntityID> {
    let entity_ids = fns::get_entity_ids_near_pos(grid_pos, entities);
    for current_entity_id in entity_ids {
        let raw_entity = entities.master_list[current_entity_id.0].0.as_ref().unwrap().as_ref();
        let dist_vec = (map_pos.0 - raw_entity.x, map_pos.1 - raw_entity.y);
        let dist_vec = (dist_vec.0 / raw_entity.width, dist_vec.1 / raw_entity.height);
        let dist_to_cell_center = fns::vec_len(dist_vec);
        if dist_to_cell_center <= 0.5 {
            return Some(current_entity_id);
        }
    }
    None
}
