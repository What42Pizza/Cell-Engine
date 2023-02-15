use crate::prelude::*;
use sdl2::{EventPump, keyboard::Keycode, render::WindowCanvas};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas, dt: &Duration) -> Result<(), ProgramError> {
    
    events::process_events(program_data, event_pump, canvas)?;

    move_camera(program_data, dt)?;

    let first_cell = program_data.world.entities_list.get_mut(&0).unwrap();
    let EntityData::Cell {x_vel, y_vel} = first_cell.data;
    first_cell.x += x_vel * dt.as_secs_f64();
    first_cell.y += y_vel * dt.as_secs_f64();

    program_data.world.sync_feilds();

    Ok(())
}



pub fn move_camera(program_data: &mut ProgramData, dt: &Duration) -> Result<(), ProgramError> {
    let mut camera = &mut program_data.camera;

    if program_data.keys_pressed.contains_key(&Keycode::W) {
        camera.y -= CAMERA_SPEED / camera.zoom * dt.as_secs_f64();
    }
    if program_data.keys_pressed.contains_key(&Keycode::S) {
        camera.y += CAMERA_SPEED / camera.zoom * dt.as_secs_f64();
    }

    if program_data.keys_pressed.contains_key(&Keycode::A) {
        camera.x -= CAMERA_SPEED / camera.zoom * dt.as_secs_f64();
    }
    if program_data.keys_pressed.contains_key(&Keycode::D) {
        camera.x += CAMERA_SPEED / camera.zoom * dt.as_secs_f64();
    }

    Ok(())
}
