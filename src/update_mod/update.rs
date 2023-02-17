use std::ptr;

use crate::{prelude::*, data_mod::general_data};
use hashbrown::{raw::RawTable, hash_map::DefaultHashBuilder};
use sdl2::{EventPump, keyboard::Keycode, render::WindowCanvas};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas, dt: f64) -> Result<(), ProgramError> {
    
    events::process_events(program_data, event_pump, canvas)?;

    move_camera(program_data, dt);

    update_cells(program_data, dt);

    program_data.cells.sync_feilds();

    Ok(())
}



pub fn move_camera(program_data: &mut ProgramData, dt: f64) {
    let mut camera = &mut program_data.camera;

    if program_data.keys_pressed.contains_key(&Keycode::W) {
        camera.y -= CAMERA_SPEED / camera.zoom * dt;
    }
    if program_data.keys_pressed.contains_key(&Keycode::S) {
        camera.y += CAMERA_SPEED / camera.zoom * dt;
    }

    if program_data.keys_pressed.contains_key(&Keycode::A) {
        camera.x -= CAMERA_SPEED / camera.zoom * dt;
    }
    if program_data.keys_pressed.contains_key(&Keycode::D) {
        camera.x += CAMERA_SPEED / camera.zoom * dt;
    }

}



pub fn update_cells(program_data: &mut ProgramData, dt: f64) {
    let cells = &mut program_data.cells;
    let keys: Vec<EntityID> = cells.master_list.keys().copied().collect();

    // main update
    for current_cell_id in &keys {

        // get cells to update
        let all_cell_ids = {
            let current_cell = cells.master_list.get(current_cell_id).unwrap();
            let mut output = vec!(*current_cell_id);
            for &connected_cell_id in &current_cell.connected_cells {
                output.push(connected_cell_id);
            }
            output
        };
        let all_cell_id_refs: Vec<&u32> = all_cell_ids.iter().collect();
        let mut all_cells = cells.master_list.get_many_mut_vec(all_cell_id_refs).unwrap();
        let (mut current_cell, connected_cells) = all_cells.split_at_mut(1);
        let current_cell = current_cell.take_first_mut().unwrap();

        // drag
        let x_drag = current_cell.x_vel * current_cell.x_vel * current_cell.x_vel.signum() * CELL_DRAG_COEF;
        let y_drag = current_cell.y_vel * current_cell.y_vel * current_cell.y_vel.signum() * CELL_DRAG_COEF;
        current_cell.x_vel -= x_drag * dt;
        current_cell.y_vel -= y_drag * dt;

        // connected cells spring
        for connected_cell in connected_cells {

            // TODO: check on https://github.com/rust-lang/hashbrown/issues/332
            let dp = current_cell.pos_change(connected_cell);
            let dv = current_cell.vel_change(connected_cell);
            let dp_len = fns::vec_len(dp.0, dp.1);

            let force_from_dist = (CELL_CONNECTION_DISTANCE - dp_len) * CELL_CONNECTION_FORCE;
            let force_from_dist_x = dp.0 * force_from_dist * -1.;
            let force_from_dist_y = dp.1 * force_from_dist * -1.;
            let force_from_drag_x = (0. - dv.0) * CELL_CONNECTION_DRAG * -1.;
            let force_from_drag_y = (0. - dv.1) * CELL_CONNECTION_DRAG * -1.;
            current_cell.x_vel += (force_from_dist_x + force_from_drag_x) * dt;
            current_cell.y_vel += (force_from_dist_y + force_from_drag_y) * dt;

        }

    }

    // final physics
    for current_cell_id in &keys {

        // apply vel
        let current_cell = cells.master_list.get_mut(current_cell_id).unwrap();
        current_cell.entity.x += current_cell.x_vel * dt;
        current_cell.entity.y += current_cell.y_vel * dt;

        // constrain pos
        if current_cell.entity.x < 0.5 {
            current_cell.entity.x = 0.5;
            current_cell.x_vel = 0.;
        }
        if current_cell.entity.x > GRID_WIDTH as f64 - 0.5 {
            current_cell.entity.x = GRID_WIDTH as f64 - 0.5;
            current_cell.x_vel = 0.;
        }
        if current_cell.entity.y < 0.5 {
            current_cell.entity.y = 0.5;
            current_cell.y_vel = 0.;
        }
        if current_cell.entity.y > GRID_HEIGHT as f64 - 0.5 {
            current_cell.entity.y = GRID_HEIGHT as f64 - 0.5;
            current_cell.y_vel = 0.;
        }

    }

}
