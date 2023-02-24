use crate::prelude::*;
use sdl2::{EventPump, keyboard::Keycode, render::WindowCanvas};



pub fn update (program_data: &mut ProgramData, canvas: &WindowCanvas, events_data: EventsData, dt: f64) -> Result<(), ProgramError> {
    
    events::process_events(program_data, events_data, canvas)?;

    move_camera(program_data, dt);

    let start = Instant::now();
    update_cells(program_data, dt);
    println!("update time (ms): {}", start.elapsed().as_secs_f64() * 1000.);

    Ok(())
}



pub fn move_camera(program_data: &mut ProgramData, dt: f64) {
    let current_speed = CAMERA_SPEED / program_data.camera.zoom * dt;

    if program_data.key_is_pressed(Keycode::W) {
        program_data.camera.y -= current_speed;
    }
    if program_data.key_is_pressed(Keycode::S) {
        program_data.camera.y += current_speed;
    }

    if program_data.key_is_pressed(Keycode::A) {
        program_data.camera.x -= current_speed;
    }
    if program_data.key_is_pressed(Keycode::D) {
        program_data.camera.x += current_speed;
    }

}



pub fn update_cells(program_data: &mut ProgramData, dt: f64) {
    if program_data.frame_count < 30 {return;}

    // main update
    for i in 0..program_data.cells.master_list.len() {
        let cell_data = &program_data.cells.master_list[i];
        if cell_data.0.is_none() {continue;}
        let current_cell_id = (i, cell_data.1);
        remove_invalid_ids(current_cell_id, &mut program_data.cells);
        let (grid_x, grid_y) = match update_single_cell(current_cell_id, program_data, dt) {
            CellUpdateResult::Alive (grid_x, grid_y) => (grid_x, grid_y),
            CellUpdateResult::Killed => continue,
        };
        update_cell_by_type(current_cell_id, program_data, dt);
        update_connected_cells(current_cell_id, &mut program_data.cells, dt);
        update_nearby_cells(current_cell_id, grid_x, grid_y, &mut program_data.cells, dt);
    }

    // final physics
    for i in 0..program_data.cells.master_list.len() {
        let cell_data = &program_data.cells.master_list[i];
        if cell_data.0.is_none() {continue;}
        let current_cell_id = (i, cell_data.1);
        update_cell_final(current_cell_id, &mut program_data.cells, dt);
    }

    // sync feilds
    program_data.cells.sync_feilds();

}





pub fn remove_invalid_ids (current_cell_id: EntityID, cells: &mut EntityContainer<DoubleBuffer<Cell>>) {
    let current_cell = cells.master_list[current_cell_id.0].0.as_ref().unwrap().get_main();
    let mut id_indexes_to_remove = vec!();
    for (i, connected_cell_id) in current_cell.connected_cells.iter().enumerate().rev() {
        if !cells.id_is_valid(*connected_cell_id) {
            id_indexes_to_remove.push(i);
        }
    }
    let current_cell = cells.master_list[current_cell_id.0].0.as_mut().unwrap().get_main_mut();
    for id_to_remove in id_indexes_to_remove {
        current_cell.connected_cells.remove(id_to_remove);
    }
}





pub enum CellUpdateResult {
    Alive (usize, usize),
    Killed,
}

pub fn update_single_cell (current_cell_id: EntityID, program_data: &mut ProgramData, dt: f64) -> CellUpdateResult {
    
    let (cell_main, cell_alt) = program_data.cells.master_list[current_cell_id.0].0.as_mut().unwrap().get_both();
    let output = (cell_alt.entity.x as usize, cell_alt.entity.y as usize);

    //-----------------------//
    //        ALWAYS:        //
    //-----------------------//

    // drag
    let x_drag = cell_alt.x_vel * cell_alt.x_vel * cell_alt.x_vel.signum() * CELL_DRAG_COEF;
    let y_drag = cell_alt.y_vel * cell_alt.y_vel * cell_alt.y_vel.signum() * CELL_DRAG_COEF;
    cell_main.x_vel -= x_drag * dt;
    cell_main.y_vel -= y_drag * dt;

    // constrain pos
    if cell_alt.entity.x < 0.5 {
        let dist = 0.5 - cell_alt.entity.x;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_main.x_vel += force * dt;
    }
    if cell_alt.entity.x > GRID_WIDTH as f64 - 0.5 {
        let dist = cell_alt.entity.x - GRID_WIDTH as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_main.x_vel += force * dt;
    }
    if cell_alt.entity.y < 0.5 {
        let dist = 0.5 - cell_alt.entity.y;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_main.y_vel += force * dt;
    }
    if cell_alt.entity.y > GRID_HEIGHT as f64 - 0.5 {
        let dist = cell_alt.entity.y - GRID_HEIGHT as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_main.y_vel += force * dt;
    }

    // dying
    cell_main.is_active = cell_alt.energy >= 0.;
    cell_main.entity.should_be_removed = cell_alt.health <= 0.;
    if cell_alt.entity.should_be_removed {
        program_data.food.add_entity(Food::from_cell(cell_alt));
        return CellUpdateResult::Killed;
    }

    if !cell_alt.is_active {return CellUpdateResult::Alive(output.0, output.1);}

    //--------------------------//
    //        IF ACTIVE:        //
    //--------------------------//

    // energy drain
    cell_main.energy -= CELL_ENERGY_USE_RATE * dt;

    // healing
    if cell_alt.health < 1. {
        let heal_amount = (1. - cell_alt.health).min(CELL_HEALING_RATE);
        cell_main.health += heal_amount * dt;
        cell_main.energy -= heal_amount * CELL_HEALING_ENERGY_COST * dt;
        cell_main.material -= heal_amount * CELL_HEALING_MATERIAL_COST * dt;
    }

    CellUpdateResult::Alive(output.0, output.1)
}





pub fn update_cell_by_type (current_cell_id: EntityID, program_data: &mut ProgramData, dt: f64) {
    let (cell_main, cell_alt) = program_data.cells.master_list[current_cell_id.0].0.as_mut().unwrap().get_both();
    if !cell_alt.is_active {return;}
    match &mut cell_main.raw_cell {

        RawCell::Fat (cell_data) => {
            // transfer logic
            if cell_alt.energy > cell_data.energy_store_threshold {
                let transfer_amount = (cell_alt.energy - cell_data.energy_release_threshold).min(cell_data.energy_store_rate) * dt;
                cell_main.energy -= transfer_amount;
                cell_data.extra_energy += transfer_amount;
            } else if cell_alt.energy < cell_data.energy_release_threshold {
                let transfer_amount = (cell_data.energy_store_threshold - cell_alt.energy).min(cell_data.energy_release_rate) * dt;
                cell_main.energy += transfer_amount;
                cell_data.extra_energy -= transfer_amount;
            }
            if cell_alt.material > cell_data.material_store_threshold {
                let transfer_amount = (cell_alt.material - cell_data.material_release_threshold).min(cell_data.material_store_rate) * dt;
                cell_main.material -= transfer_amount;
                cell_data.extra_energy += transfer_amount;
            } else if cell_alt.material < cell_data.material_release_threshold {
                let transfer_amount = (cell_data.material_store_threshold - cell_alt.material).min(cell_data.material_release_rate) * dt;
                cell_main.material += transfer_amount;
                cell_data.extra_energy -= transfer_amount;
            }
        }

        RawCell::Photosynthesiser => {
            if cell_alt.energy >= 1.0 {return;}
            let photosynthesis_amount = (1.0 - cell_alt.energy).min(CELL_PHOTOSYNTHESISER_RATE) * dt;
            cell_main.energy += photosynthesis_amount;
        }

    }
}





pub fn update_connected_cells (current_cell_id: EntityID, cells: &mut EntityContainer<DoubleBuffer<Cell>>, dt: f64) {

    // get cells
    let all_cell_ids = {
        let cell_main = cells.master_list[current_cell_id.0].0.as_mut().unwrap().get_main();
        let mut output = vec!(current_cell_id);
        for &connected_cell_id in &cell_main.connected_cells {
            output.push(connected_cell_id);
        }
        output
    };
    let (current_cell, connected_cells) = get_current_and_others(&all_cell_ids, cells);
    let (cell_main, cell_alt) = current_cell.get_both();
    let connected_cells: Vec<(&mut Cell, &Cell)> = connected_cells.into_iter()
        .map(DoubleBuffer::get_both)
        .collect();

    // connected cells
    for (connected_cell_main, connected_cell_alt) in connected_cells {

        //-----------------------//
        //        ALWAYS:        //
        //-----------------------//

        // spring
        let dp = cell_alt.pos_change_to(connected_cell_alt);
        let dv = fns::move_point_to_line(cell_alt.vel_change_to(connected_cell_alt), dp);
        let dp_len = fns::vec_len(dp);
        let dv_len = fns::vec_len(dv);
        let force_from_dist = (CELL_CONNECTION_DISTANCE - dp_len) * CELL_CONNECTION_FORCE;
        let force_from_dist_x = dp.0 * force_from_dist * -1.;
        let force_from_dist_y = dp.1 * force_from_dist * -1.;
        let force_from_drag_x = dv.0 * dv_len * CELL_CONNECTION_DRAG;
        let force_from_drag_y = dv.1 * dv_len * CELL_CONNECTION_DRAG;
        cell_main.x_vel += (force_from_dist_x + force_from_drag_x) * dt;
        cell_main.y_vel += (force_from_dist_y + force_from_drag_y) * dt;

        if !cell_main.is_active {continue;}

        //--------------------------//
        //        IF ACTIVE:        //
        //--------------------------//

        // transfers
        if connected_cell_alt.energy < cell_alt.energy {
            let transfer_amount = (cell_alt.energy - connected_cell_alt.energy) * CELL_ENERGY_TRANSFER_RATE * dt;
            cell_main.energy -= transfer_amount;
            connected_cell_main.energy += transfer_amount;
        }
        if connected_cell_alt.material < cell_alt.material {
            let transfer_amount = (cell_alt.material - connected_cell_alt.material) * CELL_MATERIAL_TRANSFER_RATE * dt;
            cell_main.material -= transfer_amount;
            connected_cell_main.material += transfer_amount;
        }

    }

}





pub fn update_nearby_cells (current_cell_id: EntityID, grid_x: usize, grid_y: usize, cells: &mut EntityContainer<DoubleBuffer<Cell>>, dt: f64) {

    // intersection force
    let mut all_cell_ids = fns::get_entity_ids_near_pos((grid_x, grid_y), cells);
    let current_cell_id = fns::find_item_index_custom(&all_cell_ids, |id| id.0 == current_cell_id.0).unwrap();
    all_cell_ids.swap(0, current_cell_id);
    let (current_cell, nearby_cells) = get_current_and_others(&all_cell_ids, cells);
    let (cell_main, cell_alt) = current_cell.get_both();
    let nearby_cells: Vec<(&mut Cell, &Cell)> = nearby_cells.into_iter()
        .map(DoubleBuffer::get_both)
        .collect();
    for (other_cell_main, other_cell_alt) in nearby_cells {
        let dist_vec = cell_alt.pos_change_to(other_cell_alt);
        let dist = fns::vec_len(dist_vec);
        if dist > 1. {continue;}
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        let force_vec = (dist_vec.0 * force, dist_vec.1 * force);
        cell_main.x_vel -= force_vec.0 * dt;
        cell_main.y_vel -= force_vec.1 * dt;
    }

}





pub fn update_cell_final (current_cell_id: EntityID, cells: &mut EntityContainer<DoubleBuffer<Cell>>, dt: f64) {
    let cell_main = cells.master_list[current_cell_id.0].0.as_mut().unwrap().get_main_mut();

    // apply vel
    cell_main.entity.x += cell_main.x_vel * dt;
    cell_main.entity.y += cell_main.y_vel * dt;
    if cell_main.entity.x.is_nan() || cell_main.entity.y.is_nan() {panic!("nan pos")}
    if cell_main.entity.x.is_infinite() || cell_main.entity.y.is_infinite() {panic!("infinite pos")}

    // clamp
    cell_main.entity.x = cell_main.entity.x.clamp(0., GRID_WIDTH  as f64 - 0.000001);
    cell_main.entity.y = cell_main.entity.y.clamp(0., GRID_HEIGHT as f64 - 0.000001);

    // swap
    //cells.master_list[current_cell_id.0].0.as_mut().unwrap().swap();
    let cell_buffer = cells.master_list[current_cell_id.0].0.as_mut().unwrap();
    cell_buffer.b = cell_buffer.a.clone();

}





pub fn get_current_and_others<'a, T: Entity> (ids: &[EntityID], entities: &'a mut EntityContainer<T>) -> (&'a mut T, Vec<&'a mut T>) {
    let id_indexes: Vec<usize> = ids.iter()
        .map(|v| v.0)
        .collect();
    let all_cells = fns::get_many_mut(&mut entities.master_list, &id_indexes);
    let mut all_cells: Vec<&mut T> = all_cells.into_iter()
        .map(|v| v.0.as_mut().unwrap())
        .collect();
    (all_cells.remove(0), all_cells)
}
