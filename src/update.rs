use crate::prelude::*;
use sdl2::{EventPump, event::Event, keyboard::Keycode, mouse::MouseState, render::WindowCanvas};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas, dt: &Duration) -> Result<(), ProgramError> {
    
    process_events(program_data, event_pump, canvas)?;

    move_camera(program_data, dt)?;

    program_data.world.cells.get_mut(&0).unwrap().x += 0.5 * dt.as_secs_f64();

    Ok(())
}



fn move_camera(program_data: &mut ProgramData, dt: &Duration) -> Result<(), ProgramError> {
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



fn process_events (program_data: &mut ProgramData, event_pump: &mut EventPump, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    let mouse_state = event_pump.mouse_state();
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => program_data.exit = true,
            Event::KeyDown {keycode: Some(keycode), ..} => {program_data.keys_pressed.insert(keycode, ());},
            Event::KeyUp {keycode: Some(keycode), ..} => {program_data.keys_pressed.remove(&keycode);},
            Event::MouseWheel {y, ..} => process_mouse_wheel(program_data, y, &mouse_state, canvas)?,
            _ => {}
        }
    }

    Ok(())
}



fn process_mouse_wheel (program_data: &mut ProgramData, y: i32, mouse_state: &MouseState, canvas: &WindowCanvas) -> Result<(), ProgramError> {
    let canvas_size = canvas.output_size()?;
    let mouse_pos = (mouse_state.x(), mouse_state.y());
    let start_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.zoom *= SCROLL_SPEED.pow(y as f64);
    let end_grid_pos = fns::convert_screen_to_grid(mouse_pos, &program_data.camera, canvas_size);
    program_data.camera.x -= end_grid_pos.0 - start_grid_pos.0;
    program_data.camera.y -= end_grid_pos.1 - start_grid_pos.1;
    Ok(())
}
