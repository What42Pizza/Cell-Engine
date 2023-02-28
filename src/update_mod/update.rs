use crate::prelude::*;
use sdl2::{keyboard::Keycode, render::WindowCanvas};



static mut TOTAL_TIME: f64 = 0.0;

pub fn update (program_data: &mut ProgramData, canvas: &WindowCanvas, events_data: EventsData, dt: f64) -> Result<(), ProgramError> {
    
    events::process_events(program_data, events_data, canvas)?;

    move_camera(program_data, dt);

    let start = Instant::now();
    update_cells(program_data, dt);
    //println!("total: update time (ms): {}\n\n\n", start.elapsed().as_secs_f64() * 1000.);

    unsafe {
        TOTAL_TIME += start.elapsed().as_secs_f64();
    }
    if program_data.frame_count == 300 {
        unsafe {
            println!("{}", TOTAL_TIME);
        }
        panic!("intentional stop to analyize data");
    }

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

    // remove invalid ids
    for i in 0..program_data.cells.master_list.len() {
        let cell_data = &program_data.cells.master_list[i];
        if cell_data.0.is_none() {continue;}
        let curr_cell_id = (i, cell_data.1);
        remove_invalid_ids(curr_cell_id, &mut program_data.cells);
    }

    let cells = &program_data.cells;

    // main update
    //let start = Instant::now();
    let all_updates: Vec<WorldUpdates> = program_data.cells.master_list.par_iter().enumerate().map(|(i, cell_data)| {
        let mut world_updates = WorldUpdates::new();
        if cell_data.0.is_none() {return world_updates;}
        let curr_cell_id = (i, cell_data.1);

        let mut cell_changes_group = CellChangesGroup::new();
        let update_result = update_single_cell(curr_cell_id, cells, &mut world_updates, &mut cell_changes_group, dt);
        if update_result == CellUpdateResult::Removed {return world_updates;}
        update_cell_by_type(curr_cell_id, cells, &mut world_updates, &mut cell_changes_group, dt);
        update_connected_cells(curr_cell_id, cells, &mut world_updates, &mut cell_changes_group, dt);
        update_nearby_cells(curr_cell_id, cells, &mut world_updates, &mut cell_changes_group, dt);

        cell_changes_group.add_self_to_world_updates(&mut world_updates, curr_cell_id);

        world_updates
    }).collect();
    //println!("main update time (ms): {}", start.elapsed().as_secs_f64() * 1000.);
    
    let (mut all_changes, mut all_additions) = (vec!(), vec!());
    for world_updates in all_updates {
        all_changes.push(world_updates.changes);
        all_additions.push(world_updates.additions);
    }

    // apply change updates
    //let start = Instant::now();
    //let mut size = 0;
    for changes in all_changes {
        //size += changes.len();
        for change in changes {
            apply_change_update(change, program_data);
        }
    }
    //println!("{size}");

    // sync feilds (& remove entities)
    program_data.cells.sync_feilds();
    program_data.food.sync_feilds();

    // apply addition updates
    for additions in all_additions {
        for addition in additions {
            apply_addition_update(addition, program_data);
        }
    }
    //println!("apply update time (ms): {}", start.elapsed().as_secs_f64() * 1000.);

}





pub fn remove_invalid_ids (curr_cell_id: EntityID, cells: &mut EntityContainer<Cell>) {
    let current_cell = cells.master_list[curr_cell_id.0].0.as_ref().unwrap();
    let mut id_indexes_to_remove = vec!();
    for (i, connected_cell_id) in current_cell.connected_cells.iter().enumerate().rev() {
        if !cells.id_is_valid(*connected_cell_id) {
            id_indexes_to_remove.push(i);
        }
    }
    let current_cell = cells.master_list[curr_cell_id.0].0.as_mut().unwrap();
    for id_to_remove in id_indexes_to_remove {
        current_cell.connected_cells.remove(id_to_remove);
    }
}





pub fn apply_change_update (update: ChangeUpdate, program_data: &mut ProgramData) {
    match update {

        ChangeUpdate::ChangeCellHealth (cell_index, value) => {
            program_data.cells.master_list[cell_index].0.as_mut().unwrap().health += value;
        }

        ChangeUpdate::ChangeCellEnergy (cell_index, value) => {
            program_data.cells.master_list[cell_index].0.as_mut().unwrap().energy += value;
        }

        ChangeUpdate::ChangeCellMaterial (cell_index, value) => {
            program_data.cells.master_list[cell_index].0.as_mut().unwrap().material += value;
        }

        ChangeUpdate::SetCellPos (cell_index, value_1, value_2) => {
            let cell = program_data.cells.master_list[cell_index].0.as_mut().unwrap();
            cell.entity.x = value_1;
            cell.entity.y = value_2;
        }

        ChangeUpdate::ChangeCellVel (cell_index, value_1, value_2) => {
            let cell = program_data.cells.master_list[cell_index].0.as_mut().unwrap();
            cell.x_vel += value_1;
            cell.y_vel += value_2;
        }

        ChangeUpdate::SetCellIsActive (cell_index, value) => {
            program_data.cells.master_list[cell_index].0.as_mut().unwrap().is_active = value;
        }

        ChangeUpdate::SetCellShouldBeRemoved (cell_index, value) => {
            program_data.cells.master_list[cell_index].0.as_mut().unwrap().entity.should_be_removed = value;
        }

        ChangeUpdate::ChangeCellFatExtraEnergy (cell_index, value) => {
            let cell = program_data.cells.master_list[cell_index].0.as_mut().unwrap();
            if let RawCell::Fat (fat_cell_data) = &mut cell.raw_cell {
                fat_cell_data.extra_energy += value;
            }
        }

        ChangeUpdate::ChangeCellFatExtraMaterial (cell_index, value) => {
            let cell = program_data.cells.master_list[cell_index].0.as_mut().unwrap();
            if let RawCell::Fat (fat_cell_data) = &mut cell.raw_cell {
                fat_cell_data.extra_material += value;
            }
        }

    }
}



pub fn apply_addition_update (update: AdditionUpdate, program_data: &mut ProgramData) {
    match update {

        AdditionUpdate::Food (food) => {
            program_data.food.add_entity(food);
        }

    }
}





#[derive(PartialEq)]
pub enum CellUpdateResult {
    Alive,
    Removed,
}

pub fn update_single_cell (curr_cell_id: EntityID, cells: &EntityContainer<Cell>, world_updates: &mut WorldUpdates, cell_changes_group: &mut CellChangesGroup, dt: f64) -> CellUpdateResult {

    let cell = cells.master_list[curr_cell_id.0].0.as_ref().unwrap();

    //-----------------------//
    //        ALWAYS:        //
    //-----------------------//

    // apply vel (it doesn't matter when this is done)
    let (mut x, mut y) = (cell.entity.x, cell.entity.y);
    x += cell.x_vel * dt;
    y += cell.y_vel * dt;
    if cell.entity.x.is_nan() || cell.entity.y.is_nan() {panic!("nan pos")}
    if cell.entity.x.is_infinite() || cell.entity.y.is_infinite() {panic!("infinite pos")}
    x = x.clamp(0., GRID_WIDTH  as f64 - 0.000001);
    y = y.clamp(0., GRID_HEIGHT as f64 - 0.000001);
    world_updates.push_change(ChangeUpdate::SetCellPos (curr_cell_id.0, x, y));

    // drag
    let x_drag = cell.x_vel * cell.x_vel * cell.x_vel.signum() * CELL_DRAG_COEF;
    let y_drag = cell.y_vel * cell.y_vel * cell.y_vel.signum() * CELL_DRAG_COEF;
    cell_changes_group.x_vel_change -= x_drag * dt;
    cell_changes_group.y_vel_change -= y_drag * dt;

    // constrain pos
    if cell.entity.x < 0.5 {
        let dist = 0.5 - cell.entity.x;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_changes_group.x_vel_change += force * dt;
    }
    if cell.entity.x > GRID_WIDTH as f64 - 0.5 {
        let dist = cell.entity.x - GRID_WIDTH as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_changes_group.x_vel_change += force * dt;
    }
    if cell.entity.y < 0.5 {
        let dist = 0.5 - cell.entity.y;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_changes_group.y_vel_change += force * dt;
    }
    if cell.entity.y > GRID_HEIGHT as f64 - 0.5 {
        let dist = cell.entity.y - GRID_HEIGHT as f64 + 0.5;
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        cell_changes_group.y_vel_change += force * dt;
    }

    // dying
    if cell.is_active && cell.energy <= 0. {
        world_updates.push_change(ChangeUpdate::SetCellIsActive (curr_cell_id.0, false));
    }
    if cell.health <= 0. {
        world_updates.push_change(ChangeUpdate::SetCellShouldBeRemoved (curr_cell_id.0, true));
        world_updates.push_addition(AdditionUpdate::Food (Food::from_cell(cell)));
        return CellUpdateResult::Removed;
    }

    if !cell.is_active {return CellUpdateResult::Alive;}

    //--------------------------//
    //        IF ACTIVE:        //
    //--------------------------//

    // energy drain
    cell_changes_group.energy_change -= CELL_ENERGY_USE_RATE * dt;

    // healing
    if cell.health < 1. {
        let heal_amount = (1. - cell.health).min(CELL_HEALING_RATE);
        world_updates.push_change(ChangeUpdate::ChangeCellHealth (curr_cell_id.0, heal_amount * dt));
        cell_changes_group.energy_change -= heal_amount * CELL_HEALING_ENERGY_COST * dt;
        cell_changes_group.material_change -= heal_amount * CELL_HEALING_MATERIAL_COST * dt;
    }

    CellUpdateResult::Alive
}





pub fn update_cell_by_type (curr_cell_id: EntityID, cells: &EntityContainer<Cell>, world_updates: &mut WorldUpdates, cell_changes_group: &mut CellChangesGroup, dt: f64) {
    let cell = cells.master_list[curr_cell_id.0].0.as_ref().unwrap();
    if !cell.is_active {return;}
    match &cell.raw_cell {

        RawCell::Fat (cell_data) => {
            // transfer logic
            if cell.energy > cell_data.energy_store_threshold {
                let transfer_amount = (cell.energy - cell_data.energy_store_threshold).min(cell_data.energy_store_rate) * dt;
                cell_changes_group.energy_change -= transfer_amount;
                world_updates.push_change(ChangeUpdate::ChangeCellFatExtraEnergy (curr_cell_id.0, transfer_amount));
            } else if cell.energy < cell_data.energy_release_threshold {
                let transfer_amount = cell_data.extra_energy.min(cell_data.energy_release_rate) * dt;
                cell_changes_group.energy_change += transfer_amount;
                world_updates.push_change(ChangeUpdate::ChangeCellFatExtraEnergy (curr_cell_id.0, transfer_amount * -1.));
            }
            if cell.material > cell_data.material_store_threshold {
                let transfer_amount = (cell.material - cell_data.material_store_threshold).min(cell_data.material_store_rate) * dt;
                cell_changes_group.material_change -= transfer_amount;
                world_updates.push_change(ChangeUpdate::ChangeCellFatExtraMaterial (curr_cell_id.0, transfer_amount));
            } else if cell.material < cell_data.material_release_threshold {
                let transfer_amount = cell_data.extra_material.min(cell_data.material_release_rate) * dt;
                cell_changes_group.material_change += transfer_amount;
                world_updates.push_change(ChangeUpdate::ChangeCellFatExtraMaterial (curr_cell_id.0, transfer_amount * -1.));
            }
        }

        RawCell::Photosynthesiser => {
            if cell.energy >= 1.0 {return;}
            let photosynthesis_amount = (1.0 - cell.energy).min(CELL_PHOTOSYNTHESISER_RATE) * dt;
            cell_changes_group.energy_change += photosynthesis_amount;
        }

    }
}





pub fn update_connected_cells (curr_cell_id: EntityID, cells: &EntityContainer<Cell>, world_updates: &mut WorldUpdates, cell_changes_group: &mut CellChangesGroup, dt: f64) {
    let cell = cells.master_list[curr_cell_id.0].0.as_ref().unwrap();

    // get connected cells
    let connected_cell_ids = &cell.connected_cells;

    // connected cells
    for &connected_cell_id in connected_cell_ids {
        let connected_cell = cells.master_list[connected_cell_id.0].0.as_ref().unwrap();

        //-----------------------//
        //        ALWAYS:        //
        //-----------------------//

        // spring
        let dp = cell.pos_change_to(connected_cell);
        let dv = fns::move_point_to_line(cell.vel_change_to(connected_cell), dp);
        let dp_len = fns::vec_len(dp);
        let dv_len = fns::vec_len(dv);
        let force_from_dist = (CELL_CONNECTION_DISTANCE - dp_len) * CELL_CONNECTION_FORCE;
        let force_from_dist_x = dp.0 * force_from_dist * -1.;
        let force_from_dist_y = dp.1 * force_from_dist * -1.;
        let force_from_drag_x = dv.0 * dv_len * CELL_CONNECTION_DRAG;
        let force_from_drag_y = dv.1 * dv_len * CELL_CONNECTION_DRAG;
        cell_changes_group.x_vel_change += (force_from_dist_x + force_from_drag_x) * dt;
        cell_changes_group.y_vel_change += (force_from_dist_y + force_from_drag_y) * dt;

        if !cell.is_active {continue;}

        //--------------------------//
        //        IF ACTIVE:        //
        //--------------------------//

        // transfers
        if cell.energy > connected_cell.energy + CELL_ENERGY_TRANSFER_THRESHOLD {
            let transfer_amount = (cell.energy - connected_cell.energy) * CELL_ENERGY_TRANSFER_RATE * dt;
            cell_changes_group.energy_change -= transfer_amount;
            world_updates.push_change(ChangeUpdate::ChangeCellEnergy (connected_cell_id.0, transfer_amount));
        }
        if cell.material > connected_cell.material + CELL_MATERIAL_TRANSFER_THRESHOLD {
            let transfer_amount = (cell.material - connected_cell.material) * CELL_MATERIAL_TRANSFER_RATE * dt;
            cell_changes_group.material_change -= transfer_amount;
            world_updates.push_change(ChangeUpdate::ChangeCellMaterial (connected_cell_id.0, transfer_amount));
        }

    }

}





pub fn update_nearby_cells (curr_cell_id: EntityID, cells: &EntityContainer<Cell>, world_updates: &mut WorldUpdates, cell_changes_group: &mut CellChangesGroup, dt: f64) {
    let cell = cells.master_list[curr_cell_id.0].0.as_ref().unwrap();
    let grid_pos = (cell.entity.current_grid_x, cell.entity.current_grid_y);

    // get nearby cells
    let mut nearby_cell_ids = fns::get_entity_ids_near_pos(grid_pos, cells);
    let curr_cell_id_index = fns::find_item_index_custom(&nearby_cell_ids, |id| id.0 == curr_cell_id.0).unwrap();
    nearby_cell_ids.swap_remove(curr_cell_id_index);

    // intersection force
    for nearby_cell_id in nearby_cell_ids {
        let other_cell = cells.master_list[nearby_cell_id.0].0.as_ref().unwrap();
        let dist_vec = cell.pos_change_to(other_cell);
        let dist = fns::vec_len(dist_vec);
        if dist > 1. {continue;}
        let force = (1. - dist).sqrt() * CELL_INTERSECTION_FORCE;
        let force_vec = (dist_vec.0 * force, dist_vec.1 * force);
        cell_changes_group.x_vel_change -= force_vec.0 * dt;
        cell_changes_group.y_vel_change -= force_vec.1 * dt;
    }

}
