use crate::prelude::*;
use sdl2::{EventPump, keyboard::Keycode, render::WindowCanvas};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas, dt: f64) -> Result<(), ProgramError> {
    //println!();
    //println!();
    //println!();
    
    events::process_events(program_data, event_pump, canvas)?;

    move_camera(program_data, dt);

    //let start = Instant::now();
    update_cells(program_data, dt);
    //println!("update time (ms): {}", start.elapsed().as_secs_f64() * 1000.);

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

    // main update (WARNING: cell positions and velocities have to stay constant here)
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





pub fn remove_invalid_ids (current_cell_id: EntityID, cells: &mut EntityContainer<Cell>) {
    let current_cell = cells.master_list[current_cell_id.0].0.as_ref().unwrap();
    let mut id_indexes_to_remove = vec!();
    for (i, connected_cell_id) in current_cell.connected_cells.iter().enumerate().rev() {
        if !cells.id_is_valid(*connected_cell_id) {
            id_indexes_to_remove.push(i);
        }
    }
    let current_cell = cells.master_list[current_cell_id.0].0.as_mut().unwrap();
    for id_to_remove in id_indexes_to_remove {
        current_cell.connected_cells.remove(id_to_remove);
    }
}





pub enum CellUpdateResult {
    Alive (usize, usize),
    Killed,
}

pub fn update_single_cell (current_cell_id: EntityID, program_data: &mut ProgramData, dt: f64) -> CellUpdateResult {
    
    let current_cell = program_data.cells.master_list[current_cell_id.0].0.as_mut().unwrap();
    let output = (current_cell.entity.x as usize, current_cell.entity.y as usize);

    //-----------------------//
    //        ALWAYS:        //
    //-----------------------//

    // drag
    let x_drag = current_cell.x_vel * current_cell.x_vel * current_cell.x_vel.signum() * CELL_DRAG_COEF;
    let y_drag = current_cell.y_vel * current_cell.y_vel * current_cell.y_vel.signum() * CELL_DRAG_COEF;
    current_cell.x_vel_copy -= x_drag * dt;
    current_cell.y_vel_copy -= y_drag * dt;

    // constrain pos
    if current_cell.entity.x < 0.5 {
        let dist = 0.5 - current_cell.entity.x;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        current_cell.x_vel_copy += force * dt;
    }
    if current_cell.entity.x > GRID_WIDTH as f64 - 0.5 {
        let dist = current_cell.entity.x - GRID_WIDTH as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        current_cell.x_vel_copy += force * dt;
    }
    if current_cell.entity.y < 0.5 {
        let dist = 0.5 - current_cell.entity.y;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        current_cell.y_vel_copy += force * dt;
    }
    if current_cell.entity.y > GRID_HEIGHT as f64 - 0.5 {
        let dist = current_cell.entity.y - GRID_HEIGHT as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        current_cell.y_vel_copy += force * dt;
    }

    // dying
    current_cell.is_active = current_cell.energy >= 0.;
    current_cell.entity.should_be_removed = current_cell.health <= 0.;
    if current_cell.entity.should_be_removed {
        program_data.food.add_entity(Food::from_cell(current_cell));
        return CellUpdateResult::Killed;
    }

    if !current_cell.is_active {return CellUpdateResult::Alive(output.0, output.1);}

    //--------------------------//
    //        IF ACTIVE:        //
    //--------------------------//

    // energy drain
    current_cell.energy -= CELL_ENERGY_USE_RATE * dt;

    // healing
    if current_cell.health < 1. {
        let heal_amount = (1. - current_cell.health).min(CELL_HEALING_RATE);
        current_cell.health += heal_amount * dt;
        current_cell.energy -= heal_amount * CELL_HEALING_ENERGY_COST * dt;
        current_cell.material -= heal_amount * CELL_HEALING_MATERIAL_COST * dt;
    }

    CellUpdateResult::Alive(output.0, output.1)
}





pub fn update_cell_by_type (current_cell_id: EntityID, program_data: &mut ProgramData, dt: f64) {
    let current_cell = program_data.cells.master_list[current_cell_id.0].0.as_mut().unwrap();
    if !current_cell.is_active {return;}
    match &mut current_cell.raw_cell {

        RawCell::Fat (cell_data) => {
            // transfer logic
            if current_cell.energy > cell_data.energy_store_threshold {
                let transfer_amount = (current_cell.energy - cell_data.energy_release_threshold).min(cell_data.energy_store_rate) * dt;
                current_cell.energy -= transfer_amount;
                cell_data.extra_energy += transfer_amount;
            } else if current_cell.energy < cell_data.energy_release_threshold {
                let transfer_amount = (cell_data.energy_store_threshold - current_cell.energy).min(cell_data.energy_release_rate) * dt;
                current_cell.energy += transfer_amount;
                cell_data.extra_energy -= transfer_amount;
            }
            if current_cell.material > cell_data.material_store_threshold {
                let transfer_amount = (current_cell.material - cell_data.material_release_threshold).min(cell_data.material_store_rate) * dt;
                current_cell.material -= transfer_amount;
                cell_data.extra_energy += transfer_amount;
            } else if current_cell.material < cell_data.material_release_threshold {
                let transfer_amount = (cell_data.material_store_threshold - current_cell.material).min(cell_data.material_release_rate) * dt;
                current_cell.material += transfer_amount;
                cell_data.extra_energy -= transfer_amount;
            }
        }

        RawCell::Photosynthesiser => {
            if current_cell.energy >= 1.0 {return;}
            let photosynthesis_amount = (1.0 - current_cell.energy).min(CELL_PHOTOSYNTHESISER_RATE) * dt;
            current_cell.energy += photosynthesis_amount;
        }

    }
}





pub fn update_connected_cells (current_cell_id: EntityID, cells: &mut EntityContainer<Cell>, dt: f64) {

    // get cells
    let all_cell_ids = {
        let current_cell = cells.master_list[current_cell_id.0].0.as_mut().unwrap();
        let mut output = vec!(current_cell_id);
        for &connected_cell_id in &current_cell.connected_cells {
            output.push(connected_cell_id);
        }
        output
    };
    let (current_cell, connected_cells) = get_current_and_others(&all_cell_ids, cells);

    // connected cells
    for connected_cell in connected_cells {

        //-----------------------//
        //        ALWAYS:        //
        //-----------------------//

        // spring
        let dp = current_cell.pos_change_to(connected_cell);
        let dv = fns::move_point_to_line(current_cell.vel_change_to(connected_cell), dp);
        let dp_len = fns::vec_len(dp);
        let dv_len = fns::vec_len(dv);
        let force_from_dist = (CELL_CONNECTION_DISTANCE - dp_len) * CELL_CONNECTION_FORCE;
        let force_from_dist_x = dp.0 * force_from_dist * -1.;
        let force_from_dist_y = dp.1 * force_from_dist * -1.;
        let force_from_drag_x = dv.0 * dv_len * CELL_CONNECTION_DRAG;
        let force_from_drag_y = dv.1 * dv_len * CELL_CONNECTION_DRAG;
        current_cell.x_vel_copy += (force_from_dist_x + force_from_drag_x) * dt;
        current_cell.y_vel_copy += (force_from_dist_y + force_from_drag_y) * dt;

        if !current_cell.is_active {continue;}

        //--------------------------//
        //        IF ACTIVE:        //
        //--------------------------//

        // transfers
        if connected_cell.energy < current_cell.energy {
            let transfer_amount = (current_cell.energy - connected_cell.energy) * CELL_ENERGY_TRANSFER_RATE * dt;
            current_cell.energy -= transfer_amount;
            connected_cell.energy += transfer_amount;
        }
        if connected_cell.material < current_cell.material {
            let transfer_amount = (current_cell.material - connected_cell.material) * CELL_MATERIAL_TRANSFER_RATE * dt;
            current_cell.material -= transfer_amount;
            connected_cell.material += transfer_amount;
        }

    }

}





pub fn update_nearby_cells (current_cell_id: EntityID, grid_x: usize, grid_y: usize, cells: &mut EntityContainer<Cell>, dt: f64) {

    // intersection force
    let mut all_cell_ids = fns::get_entity_ids_near_pos((grid_x, grid_y), cells);
    let current_cell_id = fns::find_item_index_custom(&all_cell_ids, |id| id.0 == current_cell_id.0).unwrap();
    all_cell_ids.swap(0, current_cell_id);
    let (current_cell, nearby_cells) = get_current_and_others(&all_cell_ids, cells);
    for other_cell in nearby_cells {
        let dist_vec = current_cell.pos_change_to(other_cell);
        let dist = fns::vec_len(dist_vec);
        if dist > 1. {continue;}
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        let force_vec = (dist_vec.0 * force, dist_vec.1 * force);
        current_cell.x_vel_copy -= force_vec.0 * dt;
        current_cell.y_vel_copy -= force_vec.1 * dt;
    }

}





pub fn update_cell_final (current_cell_id: EntityID, cells: &mut EntityContainer<Cell>, dt: f64) {
    let current_cell = cells.master_list[current_cell_id.0].0.as_mut().unwrap();

    // use copy as actual
    current_cell.x_vel = current_cell.x_vel_copy;
    current_cell.y_vel = current_cell.y_vel_copy;

    // apply vel
    current_cell.entity.x += current_cell.x_vel * dt;
    current_cell.entity.y += current_cell.y_vel * dt;
    if current_cell.entity.x.is_nan() || current_cell.entity.y.is_nan() {panic!("nan pos")}
    if current_cell.entity.x.is_infinite() || current_cell.entity.y.is_infinite() {panic!("infinite pos")}

    // clamp
    current_cell.entity.x = current_cell.entity.x.clamp(0., GRID_WIDTH  as f64 - 0.000001);
    current_cell.entity.y = current_cell.entity.y.clamp(0., GRID_HEIGHT as f64 - 0.000001);

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
